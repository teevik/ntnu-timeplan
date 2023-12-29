use crate::caching::activities_cache::ActivitiesCache;
use crate::caching::courses_cache::CoursesCache;
use crate::caching::semesters_cache::SemestersCache;
use std::sync::Arc;

pub mod caching;
pub mod calendar;
pub mod error;
pub mod fetch;
pub mod router;
pub mod shared_types;

#[derive(Clone)]
pub struct AppState {
    pub activities_cache: Arc<ActivitiesCache>,
    pub courses_cache: Arc<CoursesCache>,
    pub semesters_cache: Arc<SemestersCache>,
}

impl AppState {
    pub async fn new(reqwest_client: &reqwest::Client) -> anyhow::Result<Self> {
        let activities_cache: ActivitiesCache = ActivitiesCache::new(reqwest_client.clone());
        let courses_cache = CoursesCache::new(reqwest_client.clone()).await;
        let semesters_cache = SemestersCache::new(reqwest_client.clone()).await?;

        Ok(Self {
            activities_cache: Arc::new(activities_cache),
            courses_cache: Arc::new(courses_cache),
            semesters_cache: Arc::new(semesters_cache),
        })
    }
}
