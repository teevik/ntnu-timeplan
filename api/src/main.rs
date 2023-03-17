#![feature(default_free_fn)]
#![feature(async_closure)]

use crate::caching::activities_cache::ActivitiesCache;
use crate::caching::courses_cache::CoursesCache;
use crate::caching::semesters_cache::SemestersCache;
use axum::routing::get;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

mod caching;
mod calendar;
mod fetch;

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
        Err(_) => 8080,
    };

    let router = Router::new()
        .route("/", get(async || "halla"))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
