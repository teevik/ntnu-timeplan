use crate::data::activities_for_courses::{get_activities_for_courses, ActivitiesForCourse};
use crate::data::course::CourseIdentifier;
use crate::data::semester::Semester;
use crate::fetch::activities::fetch_activities;
use itertools::Itertools;
use mini_moka::sync::Cache;
use std::sync::Arc;
use time::ext::NumericalStdDuration;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ActivitiesCacheKey {
    pub semester: Semester,
    pub course_identifier: CourseIdentifier,
}

pub struct ActivitiesCache {
    cache: Cache<ActivitiesCacheKey, Arc<ActivitiesForCourse>>,
}

impl ActivitiesCache {
    pub fn new() -> Self {
        let cache = Cache::builder()
            .max_capacity(500)
            .time_to_live(2.std_hours())
            .build();

        Self { cache }
    }

    pub async fn get_or_fetch_activities(
        &self,
        semester: &Semester,
        course_identifiers: impl IntoIterator<Item = &CourseIdentifier>,
        client: &reqwest::Client,
    ) -> anyhow::Result<Vec<(CourseIdentifier, Arc<ActivitiesForCourse>)>> {
        let course_identifiers = course_identifiers.into_iter().collect_vec();

        let get_cache_key = |course_identifier: CourseIdentifier| ActivitiesCacheKey {
            semester: semester.clone(),
            course_identifier,
        };

        let activities_for_courses = course_identifiers
            .iter()
            .cloned()
            .map(|course_identifier| {
                let cache_key = get_cache_key(course_identifier.clone());
                let activities_for_course = self.cache.get(&cache_key)?;

                Some((cache_key.course_identifier, activities_for_course))
            })
            .collect::<Option<Vec<(CourseIdentifier, Arc<ActivitiesForCourse>)>>>();

        let cached_activities_for_courses = match activities_for_courses {
            Some(activities_for_courses) => activities_for_courses,
            None => {
                let activities = fetch_activities(semester, course_identifiers, client).await?;
                let activities_for_courses = get_activities_for_courses(activities);

                let cached_activities_for_courses = activities_for_courses
                    .into_iter()
                    .map(|(course_identifier, activities_for_course)| {
                        let cache_key = get_cache_key(course_identifier.clone());
                        let activities_for_course = Arc::new(activities_for_course);
                        self.cache.insert(cache_key, activities_for_course.clone());

                        (course_identifier, activities_for_course)
                    })
                    .collect_vec();

                cached_activities_for_courses
            }
        };

        Ok(cached_activities_for_courses)
    }
}
