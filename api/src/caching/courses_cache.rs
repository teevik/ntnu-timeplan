use crate::error::AppResult;
use crate::fetch::courses::fetch_courses;
use crate::shared_types::Course;
use mini_moka::sync::Cache;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use time::ext::NumericalStdDuration;
use tracing::info;

#[derive(Debug)]
pub struct CoursesCache {
    client: Client,
    cache: Cache<String, Arc<HashMap<String, Course>>>,
}

impl CoursesCache {
    pub async fn new(client: Client) -> Self {
        let cache = Cache::builder().time_to_live(2.std_weeks()).build();

        Self { client, cache }
    }

    pub async fn get_or_fetch(&self, semester: String) -> AppResult<Arc<HashMap<String, Course>>> {
        if let Some(cache_result) = self.cache.get(&semester) {
            return Ok(cache_result);
        }

        info!("Fetching courses for {semester}");

        let courses = fetch_courses(&semester, &self.client).await?;
        let courses = Arc::new(courses);

        self.cache.insert(semester.clone(), courses.clone());
        Ok(courses)
    }
}
