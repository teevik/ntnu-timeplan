use icalendar::{Component, Event, EventLike};
use itertools::Itertools;

use crate::shared_types::{Activity, Room};

pub fn activity_to_event(activity: &Activity) -> Event {
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
