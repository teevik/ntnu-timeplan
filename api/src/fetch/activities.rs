use chrono::{DateTime, Utc};
use itertools::Itertools;
use scraper::{Html, Selector};
use serde::Deserialize;

use crate::shared_types::{Activity, CourseIdentifier, Room, StaffMember};

pub async fn fetch_activities<'a>(
    course_identifier: &CourseIdentifier,
    client: &reqwest::Client,
) -> anyhow::Result<Vec<Activity>> {
    let CourseIdentifier {
        course_code,
        course_term,
        semester,
    } = course_identifier;

    let query = vec![
        ("type", "course".to_owned()),
        ("sem", semester.into()),
        ("id[]", format!("{},{}", course_code, course_term)),
    ];

    let res = client
        .get("https://tp.educloud.no/ntnu/timeplan/index.php?type=course")
        .query(&query)
        .send()
        .await?;

    let html = res.text().await?;
    let document = Html::parse_document(&html);

    let selector = Selector::parse("script#data-js").unwrap();
    let Some(element) = document.select(&selector).next() else {return Ok(Vec::new())};
    let data = element.inner_html();

    #[derive(Debug, Deserialize)]
    struct ParsedRoom {
        #[serde(rename = "roomname")]
        pub name: String,

        #[serde(rename = "buildingname")]
        pub building_name: String,

        #[serde(rename = "roomurl")]
        pub url: String,
    }

    impl From<ParsedRoom> for Room {
        fn from(parsed_room: ParsedRoom) -> Room {
            Room {
                name: parsed_room.name,
                building_name: parsed_room.building_name,
                url: parsed_room.url,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct ParsedStaffMember {
        #[serde(rename = "firstname")]
        pub first_name: String,

        #[serde(rename = "lastname")]
        pub last_name: String,
    }

    impl From<ParsedStaffMember> for StaffMember {
        fn from(parsed_staff_member: ParsedStaffMember) -> StaffMember {
            StaffMember {
                first_name: parsed_staff_member.first_name,
                last_name: parsed_staff_member.last_name,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct ParsedActivity {
        #[serde(rename = "eventid")]
        pub id: String,

        #[serde(rename = "courseid")]
        pub course_code: String,

        #[serde(rename = "weeknr")]
        pub week: i32,

        #[serde(rename = "dtstart")]
        pub start: String,

        #[serde(rename = "dtend")]
        pub end: String,

        #[serde(rename = "teaching-title")]
        pub title: String,

        #[serde(rename = "summary")]
        pub summary: String,

        #[serde(rename = "staffs")]
        pub staff_members: Option<Vec<ParsedStaffMember>>,

        #[serde(rename = "studentgroups")]
        pub student_groups: Option<Vec<String>>,

        #[serde(rename = "room")]
        pub rooms: Option<Vec<ParsedRoom>>,
    }

    fn convert_activity(parsed_activity: ParsedActivity) -> anyhow::Result<Activity> {
        fn parse_date_time(input: String) -> anyhow::Result<DateTime<Utc>> {
            let date_time = DateTime::parse_from_str(&input, "%FT%T%#z")?.into();

            Ok(date_time)
        }

        fn vec_into<From: Into<To>, To>(vec: Vec<From>) -> Vec<To> {
            vec.into_iter().map_into().collect()
        }

        let course_code = parsed_activity.course_code;

        let activity = Activity {
            id: parsed_activity.id,
            course_code,
            week: parsed_activity.week,
            start: parse_date_time(parsed_activity.start)?,
            end: parse_date_time(parsed_activity.end)?,
            title: parsed_activity.title,
            summary: parsed_activity.summary,
            staff_members: parsed_activity
                .staff_members
                .map(vec_into)
                .unwrap_or_default(),
            student_groups: parsed_activity.student_groups.unwrap_or_default(),
            rooms: parsed_activity.rooms.map(vec_into).unwrap_or_default(),
        };

        Ok(activity)
    }

    let parsed_activities = serde_json::from_str::<Vec<ParsedActivity>>(&data)?;

    let activities = parsed_activities
        .into_iter()
        .map(convert_activity)
        .try_collect()?;

    Ok(activities)
}
