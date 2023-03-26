#![feature(default_free_fn)]
#![feature(async_closure)]

use crate::caching::activities_cache::ActivitiesCache;
use crate::caching::courses_cache::CoursesCache;
use crate::caching::semesters_cache::SemestersCache;
use crate::handlers::activities::activities_handler;
use crate::handlers::calendar::calendar_handler;
use crate::handlers::courses::courses_handler;
use crate::handlers::semesters::semesters_handler;
use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
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

mod app_error;
mod caching;
mod fetch;
mod handlers;
mod shared_types;

#[derive(Clone)]
pub struct AppState {
    pub activities_cache: Arc<ActivitiesCache>,
    pub courses_cache: Arc<CoursesCache>,
    pub semesters_cache: Arc<SemestersCache>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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
        Err(_) => 3000,
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
        .route("/semesters", get(semesters_handler))
        .route("/courses", get(courses_handler))
        .route("/activities", get(activities_handler))
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
