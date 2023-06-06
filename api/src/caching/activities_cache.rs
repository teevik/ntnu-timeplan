use crate::{
    fetch::activities::fetch_activities,
    shared_types::{Activity, CourseIdentifier},
};
use mini_moka::sync::Cache;
use reqwest::Client;
use std::sync::Arc;
use time::ext::NumericalStdDuration;
use tracing::info;

pub struct ActivitiesCache {
    client: Client,
    cache: Cache<CourseIdentifier, Arc<Vec<Activity>>>,
}

impl ActivitiesCache {
    pub fn new(client: Client) -> Self {
        let cache = Cache::builder()
            .max_capacity(500)
            .time_to_live(2.std_hours())
            .build();

        Self { client, cache }
    }

    pub async fn get_or_fetch(
        &self,
        course_identifier: CourseIdentifier,
    ) -> anyhow::Result<Arc<Vec<Activity>>> {
        if let Some(cache_result) = self.cache.get(&course_identifier) {
            return Ok(cache_result);
        }

        info!("Fetching activities for {:?}", &course_identifier);
        let activities = fetch_activities(&course_identifier, &self.client).await?;
        let activities = Arc::new(activities);

        self.cache.insert(course_identifier, activities.clone());

        Ok(activities)
    }
}
