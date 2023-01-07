use crate::data::activity::{Activity, Room};
use crate::data::course::CourseIdentifier;
use crate::fetch::activities::fetch_activities;
use axum::extract::State;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use icalendar::{Calendar, Component, Event, EventLike};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task;
use tokio::time::interval;
use tracing::info;

mod data;
mod fetch;

async fn fetch_calendar() -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    mod courses {
        pub const BMA1020: &str = "BMA1020";
        pub const EXPH0300: &str = "EXPH0300";
        pub const PROG1003: &str = "PROG1003";
        pub const PROG1004: &str = "PROG1004";
    }

    let target_student_groups_for_course = HashMap::from([
        (courses::BMA1020, HashSet::from(["BPROG_2_"])),
        (courses::EXPH0300, HashSet::from(["BPROG_2_"])), /*, "EX.PHIL GJØVIK"*/
        (courses::PROG1003, HashSet::from(["BPROG_2_"])),
        (courses::PROG1004, HashSet::from(["BPROG_2_"])),
    ]);

    let activities = fetch_activities(
        [
            CourseIdentifier::new(courses::BMA1020, 1),
            CourseIdentifier::new(courses::EXPH0300, 1),
            CourseIdentifier::new(courses::PROG1003, 1),
            CourseIdentifier::new(courses::PROG1004, 1),
        ],
        &client,
    )
    .await?;

    #[derive(Debug)]
    pub struct ActivitiesForCourse<'a> {
        pub all_student_groups: HashSet<&'a String>,
        pub activities: Vec<&'a Activity>,
    }

    fn get_activities_for_courses(activities: &[Activity]) -> HashMap<&str, ActivitiesForCourse> {
        let mut activities_for_courses = HashMap::<&str, ActivitiesForCourse>::new();

        for activity in activities {
            let empty_activities_for_course = || ActivitiesForCourse {
                all_student_groups: Default::default(),
                activities: Default::default(),
            };

            let activities_for_course = activities_for_courses
                .entry(&activity.course_code)
                .or_insert_with(|| empty_activities_for_course());

            activities_for_course
                .all_student_groups
                .extend(activity.student_groups.iter());

            activities_for_course.activities.push(activity);
        }

        activities_for_courses
    }

    let activities_for_courses = get_activities_for_courses(&activities);

    // for (course_code, activities_for_course) in &activities_for_courses {
    //     dbg!(course_code, &activities_for_course.all_student_groups);
    // }

    fn activity_to_event(activity: &Activity) -> Event {
        let mut event = Event::new();

        event.uid(&activity.id);

        event.summary(&format!("{} {}", &activity.course_code, &activity.title));
        event.starts(activity.start);
        event.ends(activity.end);

        let format_room_name = |room: &Room| format!("{} ({})", room.name, room.building_name);

        if let Some(primary_room) = activity.rooms.first() {
            event.location(&format_room_name(primary_room));
        }

        let mut description = format!("{} {}", &activity.course_code, &activity.title);

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

    let events = activities_for_courses
        .into_iter()
        .flat_map(|(course_code, activities_for_course)| {
            let target_student_groups = &target_student_groups_for_course[course_code];

            activities_for_course
                .activities
                .into_iter()
                .filter(|activity| {
                    let includes_target_group =
                        target_student_groups.iter().any(|&target_student_group| {
                            activity.student_groups.contains(target_student_group)
                        });

                    includes_target_group
                })
        })
        .map(|activity| activity_to_event(activity));

    let calendar = Calendar::from_iter(events).to_string();

    Ok(calendar.to_string())
}

#[derive(Clone)]
struct ServerState {
    pub calendar: Arc<RwLock<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>()?,
        Err(_) => 8080,
    };

    let calendar = fetch_calendar().await?;
    let calendar = Arc::new(RwLock::new(calendar));

    {
        let calendar = calendar.clone();

        task::spawn(async move {
            let mut interval = interval(Duration::from_secs(60 * 60)); // 1 hour
            interval.tick().await; // skip first tick

            loop {
                interval.tick().await;

                info!("Refreshing calendar");

                let new_calendar = fetch_calendar().await.unwrap();

                let mut calendar_write = calendar.write().await;
                *calendar_write = new_calendar;
            }
        });
    };

    async fn get_calendar(State(state): State<ServerState>) -> String {
        let calendar = state.calendar.read().await;

        calendar.clone()
    }

    let router = Router::new()
        .route("/", get(|| async { Redirect::to("/calendar.ics") }))
        .route("/calendar.ics", get(get_calendar))
        .with_state(ServerState { calendar });

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
