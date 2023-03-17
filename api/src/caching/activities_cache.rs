use crate::fetch::activities::fetch_activities;
use mini_moka::sync::Cache;
use ntnu_timeplan_shared::{Activity, CourseIdentifier};
use reqwest::Client;
use std::sync::Arc;
use time::ext::NumericalStdDuration;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ActivitiesCacheKey {
    pub semester: String,
    pub course_identifier: CourseIdentifier,
}

pub struct ActivitiesCache {
    client: Client,
    cache: Cache<ActivitiesCacheKey, Arc<Vec<Activity>>>,
}

impl ActivitiesCache {
    pub fn new(client: Client) -> Self {
        let cache = Cache::builder()
            .max_capacity(500)
            .time_to_live(2.std_hours())
            .build();

        Self { cache, client }
    }

    pub async fn get_or_fetch_activities(
        &self,
        semester: String,
        course_identifier: CourseIdentifier,
    ) -> anyhow::Result<Arc<Vec<Activity>>> {
        let cache_key = ActivitiesCacheKey {
            semester: semester.clone(),
            course_identifier: course_identifier.clone(),
        };

        if let Some(cache_result) = self.cache.get(&cache_key) {
            return Ok(cache_result);
        }

        let activities = fetch_activities(semester, &course_identifier, &self.client).await?;
        let activities = Arc::new(activities);

        self.cache.insert(cache_key, activities.clone());

        Ok(activities)
    }
}
