#![feature(default_free_fn)]
#![feature(async_closure)]

use crate::caching::activities_cache::ActivitiesCache;
use crate::caching::courses_cache::CoursesCache;
use crate::caching::semesters_cache::SemestersCache;
use crate::handlers::activities::activities_handler;
use crate::handlers::calendar::calendar_handler;
use crate::handlers::courses::courses_handler;
use crate::handlers::encode_calendar_query::encode_calendar_query_handler;
use crate::handlers::semesters::semesters_handler;
use crate::shared_types::{
    Activity, CalendarQuery, Course, CourseIdentifier, Room, Semester, SemestersWithCurrent,
    StaffMember,
};
use axum::error_handling::HandleErrorLayer;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{BoxError, Router};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_governor::errors::display_error;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app_error;
mod caching;
mod calendar_queries;
mod fetch;
mod handlers;
mod shared_types;

#[derive(Clone)]
pub struct AppState {
    pub activities_cache: Arc<ActivitiesCache>,
    pub courses_cache: Arc<CoursesCache>,
    pub semesters_cache: Arc<SemestersCache>,
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().without_time();

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    install_tracing();
    color_eyre::install()?;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::activities::activities_handler,
            handlers::courses::courses_handler,
            handlers::semesters::semesters_handler,
            handlers::calendar::calendar_handler,
            handlers::encode_calendar_query::encode_calendar_query_handler
        ),
        components(
            schemas(Activity, CalendarQuery, Course, CourseIdentifier, Room, Semester, SemestersWithCurrent, StaffMember)
        ),
        tags(
            (name = "ntnu-timeplan-api", description = "NTNU Timeplan API")
        )
    )]
    struct ApiDoc;

    let reqwest_client = reqwest::Client::new();

    let activities_cache = ActivitiesCache::new(reqwest_client.clone());
    let courses_cache = CoursesCache::new(reqwest_client.clone()).await?;
    let semesters_cache = SemestersCache::new(reqwest_client.clone()).await?;

    let app_state = AppState {
        activities_cache: Arc::new(activities_cache),
        courses_cache: Arc::new(courses_cache),
        semesters_cache: Arc::new(semesters_cache),
    };

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>()?,
        Err(_) => 8080,
    };

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    let governor_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            display_error(e)
        }))
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        });

    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { Redirect::to("/swagger-ui") }))
        .route("/semesters", get(semesters_handler))
        .route("/courses", get(courses_handler))
        .route("/activities", get(activities_handler))
        .route(
            "/encode-calendar-query",
            post(encode_calendar_query_handler),
        )
        .route("/calendar.ics", get(calendar_handler))
        .with_state(app_state)
        .layer(governor_layer)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
