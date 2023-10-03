use crate::calendar::encode_query::encode_calendar_query;
use crate::shared_types::{CalendarQuery, CourseIdentifier};
use crate::AppState;
use std::ops::Deref;

pub fn rspc_router() -> rspc::Router<AppState> {
    let router = rspc::Router::<AppState>::new()
        .query("semesters", |t| {
            t(async move |app_state: AppState, _input: ()| {
                let semester_cache = &app_state.semesters_cache;
                let semesters = semester_cache.get_or_fetch().await?;

                // TODO see if possible to return arc to value
                Ok(semesters.deref().clone())
            })
        })
        .query("courses", |t| {
            t(async move |app_state: AppState, _input: ()| {
                let courses_cache = &app_state.courses_cache;
                let courses = courses_cache.get_or_fetch().await?;

                Ok(courses.deref().clone())
            })
        })
        .query("activities", |t| {
            t(
                async move |app_state: AppState, course_identifier: CourseIdentifier| {
                    let activities_cache = &app_state.activities_cache;

                    let activities = activities_cache.get_or_fetch(course_identifier).await?;

                    Ok(activities.deref().clone())
                },
            )
        })
        .query("encode-calendar-query", |t| {
            t(async move |_, input: Vec<CalendarQuery>| {
                let encoded_query = encode_calendar_query(&input)?;

                Ok(encoded_query)
            })
        })
        .build();

    router
}
