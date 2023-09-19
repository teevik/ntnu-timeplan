use crate::error::AppResult;
use crate::{
    calendar::{activity_to_event::activity_to_event, encode_query::decode_calendar_query},
    shared_types::Activity,
    AppState,
};
use axum::extract::{Query, State};
use futures_util::{future::try_join_all, TryFutureExt};
use icalendar::Calendar;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct HandlerQuery {
    query: String,
}

pub async fn calendar_handler(
    query: Query<HandlerQuery>,
    State(app_state): State<AppState>,
) -> AppResult<String> {
    let calendar_queries = decode_calendar_query(&query.query)?;
    let activities_cache = &app_state.activities_cache;

    #[derive(Debug)]
    struct ActivitiesWithExtra {
        activities: Arc<Vec<Activity>>,
        student_groups: Vec<String>,
        custom_name: Option<String>,
    }

    let activities = calendar_queries.into_iter().map(|query| {
        activities_cache
            .get_or_fetch(query.identifier)
            .map_ok(|activities| ActivitiesWithExtra {
                activities,
                student_groups: query.student_groups,
                custom_name: query.custom_name,
            })
    });

    let all_activities_with_student_groups: Vec<ActivitiesWithExtra> =
        try_join_all(activities).await?;

    fn includes_target_group(activity: &Activity, target_student_groups: &[String]) -> bool {
        target_student_groups
            .iter()
            .any(|target_student_group| activity.student_groups.contains(target_student_group))
    }

    let events = all_activities_with_student_groups.iter().flat_map(
        |ActivitiesWithExtra {
             activities,
             student_groups,
             custom_name,
         }| {
            activities
                .iter()
                .filter(move |activity| includes_target_group(activity, student_groups))
                .map(|activity| activity_to_event(activity, custom_name))
        },
    );

    let calendar = events.collect::<Calendar>();

    Ok(calendar.to_string())
}
