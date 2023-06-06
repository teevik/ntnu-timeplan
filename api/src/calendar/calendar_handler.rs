use crate::{
    calendar::{activity_to_event::activity_to_event, encode_query::decode_calendar_query},
    shared_types::Activity,
    AppState,
};
use futures_util::{future::try_join_all, TryFutureExt};
use icalendar::Calendar;
use poem::{
    handler,
    web::{Data, Query},
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct HandlerQuery {
    query: String,
}

#[handler]
pub async fn calendar_handler(
    query: Query<HandlerQuery>,
    state: Data<&AppState>,
) -> poem::Result<String> {
    let calendar_queries = decode_calendar_query(&query.query)?;
    let activities_cache = &state.activities_cache;

    struct ActivitiesWithStudentGroups {
        activities: Arc<Vec<Activity>>,
        student_groups: Vec<String>,
    }

    let activities = calendar_queries.into_iter().map(|query| {
        activities_cache
            .get_or_fetch(query.identifier)
            .map_ok(|activities| ActivitiesWithStudentGroups {
                activities,
                student_groups: query.student_groups,
            })
    });

    let all_activities_with_student_groups: Vec<ActivitiesWithStudentGroups> =
        try_join_all(activities).await?;

    fn includes_target_group(activity: &Activity, target_student_groups: &[String]) -> bool {
        target_student_groups
            .iter()
            .any(|target_student_group| activity.student_groups.contains(target_student_group))
    }

    let events = all_activities_with_student_groups.iter().flat_map(
        |ActivitiesWithStudentGroups {
             activities,
             student_groups,
         }| {
            activities
                .iter()
                .filter(move |activity| includes_target_group(activity, student_groups))
                .map(activity_to_event)
        },
    );

    let calendar = events.collect::<Calendar>();

    Ok(calendar.to_string())
}
