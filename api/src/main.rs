use crate::api::Api;
use crate::calendar::calendar_handler::calendar_handler;
use caching::activities_cache::ActivitiesCache;
use caching::courses_cache::CoursesCache;
use caching::semesters_cache::SemestersCache;
use poem::listener::TcpListener;
use poem::middleware::{Cors, Tracing};
use poem::{get, EndpointExt, Route};
use poem_openapi::OpenApiService;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

mod api;
mod caching;
mod calendar;
mod fetch;
mod shared_types;

#[derive(Clone)]
pub struct AppState {
    pub activities_cache: Arc<ActivitiesCache>,
    pub courses_cache: Arc<CoursesCache>,
    pub semesters_cache: Arc<SemestersCache>,
}

impl AppState {
    pub async fn new(reqwest_client: &reqwest::Client) -> anyhow::Result<Self> {
        let activities_cache: ActivitiesCache = ActivitiesCache::new(reqwest_client.clone());
        let courses_cache = CoursesCache::new(reqwest_client.clone()).await?;
        let semesters_cache = SemestersCache::new(reqwest_client.clone()).await?;

        Ok(Self {
            activities_cache: Arc::new(activities_cache),
            courses_cache: Arc::new(courses_cache),
            semesters_cache: Arc::new(semesters_cache),
        })
    }
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer();

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
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    install_tracing();

    let reqwest_client = reqwest::Client::new();

    let app_state = AppState::new(&reqwest_client).await?;

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>()?,
        Err(_) => 8080,
    };

    let api_service = OpenApiService::new(Api, "NTNU Timeplan API", env!("CARGO_PKG_VERSION"))
        .server(format!("http://0.0.0.0:{port}/api"));
    let ui = api_service.openapi_explorer();
    let spec = api_service.spec();

    let app = Route::new()
        .nest("/", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .nest("/api", api_service.with(Cors::new()).with(Tracing))
        .at("/calendar.ics", get(calendar_handler).with(Tracing))
        .data(app_state);

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on http://{}", socket_addr);

    poem::Server::new(TcpListener::bind(socket_addr))
        .run(app)
        .await?;

    Ok(())
}
