use crate::app_error::AppError;
use crate::calendar_queries::decode_calendar_query;
use crate::shared_types::{Activity, Room};
use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use futures::future::try_join_all;
use futures::TryFutureExt;
use icalendar::{Calendar, Component, Event, EventLike};
use itertools::Itertools;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::IntoParams;

fn activity_to_event(activity: &Activity) -> Event {
    let mut event = Event::new();

    event.uid(&activity.id);

    event.summary(&format!("{} {}", activity.course_code, activity.title));
    event.starts(activity.start);
    event.ends(activity.end);

    let format_room_name = |room: &Room| format!("{} ({})", room.name, room.building_name);

    if let Some(primary_room) = activity.rooms.first() {
        event.location(&format_room_name(primary_room));
    }

    let mut description = format!("{} {}", activity.course_code, activity.title);

    description += "\n\n";

    description += &activity
        .staff_members
        .iter()
        .map(|staff_member| format!("{} {}", staff_member.first_name, staff_member.last_name))
        .join(", ");

    description += "\n\n";

    description += &activity
        .rooms
        .iter()
        .map(|room| format!("{}: {}", format_room_name(room), room.url))
        .join("\n");

    event.description(&description);

    event.done()
}

#[derive(Deserialize, IntoParams)]
pub struct HandlerQuery {
    query: String,
}

#[utoipa::path(
    get,
    path = "/calendar.ics",
    params(HandlerQuery),
    responses(
        (status = 200, body = String)
    )
)]
pub async fn calendar_handler(
    handler_query: Query<HandlerQuery>,
    state: State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let calendar_queries = decode_calendar_query(&handler_query.query)?;

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
