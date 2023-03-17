use crate::fetch::courses::fetch_courses;
use ntnu_timeplan_shared::Course;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug)]
pub struct CoursesCache {
    client: Client,
    last_time_fetched: RwLock<Instant>,
    courses: RwLock<Arc<HashMap<String, Course>>>,
}

impl CoursesCache {
    pub async fn new(client: Client) -> anyhow::Result<Self> {
        let courses = fetch_courses(&client).await?;
        let last_time_fetched = Instant::now();

        Ok(Self {
            client,
            last_time_fetched: RwLock::new(last_time_fetched),
            courses: RwLock::new(Arc::new(courses)),
        })
    }

    pub async fn get_or_fetch(&self) -> anyhow::Result<Arc<HashMap<String, Course>>> {
        // const CACHE_DURATION: Duration = 1.std_weeks();
        const CACHE_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 7);

        let last_time_fetched = *self.last_time_fetched.read().await;
        let cache_out_of_date = Instant::now() > (last_time_fetched + CACHE_DURATION);

        if cache_out_of_date {
            info!("Fetching courses");

            let courses = fetch_courses(&self.client).await?;

            *self.last_time_fetched.write().await = Instant::now();
            *self.courses.write().await = Arc::new(courses);
        };

        let cached_courses = self.courses.read().await.clone();

        Ok(cached_courses)
    }
}
