use crate::caching::activities_cache::ActivitiesCache;
use crate::data::activity::{Activity, Room};
use crate::data::course::CourseIdentifier;
use icalendar::{Calendar, Component, Event, EventLike};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

fn activity_to_event(activity: &Activity) -> Event {
    let course_code = &*activity.course_identifier.course_code;

    let mut event = Event::new();

    event.uid(&activity.id);

    event.summary(&format!("{} {}", course_code, &activity.title));
    event.starts(activity.start);
    event.ends(activity.end);

    let format_room_name = |room: &Room| format!("{} ({})", room.name, room.building_name);

    if let Some(primary_room) = activity.rooms.first() {
        event.location(&format_room_name(primary_room));
    }

    let mut description = format!("{} {}", course_code, &activity.title);

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TimetableQuery {
    pub course: CourseIdentifier,
    pub target_student_groups: HashSet<String>,
}

pub async fn generate_timetable(
    semester: &str,
    queries: impl IntoIterator<Item = TimetableQuery>,
    cache: &ActivitiesCache,
) -> anyhow::Result<Calendar> {
    let queries = queries
        .into_iter()
        .map(|query| (query.course, query.target_student_groups))
        .collect::<HashMap<CourseIdentifier, HashSet<String>>>();

    let activities_for_courses = cache
        .get_or_fetch_activities(semester, queries.keys().cloned())
        .await?;

    let events = activities_for_courses
        .iter()
        .flat_map(|(course_identifier, activities_for_course)| {
            let target_student_groups = &queries[&course_identifier];

            activities_for_course.activities.iter().filter(|activity| {
                let includes_target_group =
                    target_student_groups.iter().any(|target_student_group| {
                        activity.student_groups.contains(target_student_group)
                    });

                includes_target_group
            })
        })
        .map(|activity| activity_to_event(activity));

    let calendar = Calendar::from_iter(events);

    Ok(calendar)
}
