use crate::error::AppResult;
use crate::{
    fetch::activities::fetch_activities,
    shared_types::{Activity, CourseIdentifier},
};
use mini_moka::sync::Cache;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use time::ext::NumericalStdDuration;
use tokio::time::sleep;
use tracing::info;

pub struct ActivitiesCache {
    client: Client,
    cache: Cache<CourseIdentifier, Arc<Vec<Activity>>>,
}

impl ActivitiesCache {
    pub fn new(client: Client) -> Self {
        let cache = Cache::builder().time_to_live(2.std_hours()).build();

        Self { client, cache }
    }

    pub async fn get_or_fetch(
        &self,
        course_identifier: CourseIdentifier,
    ) -> AppResult<Arc<Vec<Activity>>> {
        if let Some(cache_result) = self.cache.get(&course_identifier) {
            return Ok(cache_result);
        }

        info!("Fetching activities for {:?}", &course_identifier);

        const MAX_RETRIES: usize = 5;

        for retry in 1..=MAX_RETRIES {
            let activities = fetch_activities(&course_identifier, &self.client).await?;

            // Insert to cache and return if successful
            if !activities.is_empty() {
                let activities = Arc::new(activities);

                self.cache.insert(course_identifier, activities.clone());

                return Ok(activities);
            }

            if retry != MAX_RETRIES {
                // Sleep and retry
                sleep(Duration::from_millis(1000)).await;
                info!("Retrying to fetch activities for {:?}", &course_identifier);
            }
        }

        Ok(Arc::new(Vec::new()))
    }
}
