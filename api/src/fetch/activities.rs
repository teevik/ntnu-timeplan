use crate::data::activity::{Activity, Room, StaffMember};
use crate::data::course::{CourseCode, CourseIdentifier};
use crate::data::semester::Semester;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashSet;
use std::ops::Deref;

pub async fn fetch_activities<'a>(
    semester: &Semester,
    course_identifiers: impl IntoIterator<Item = CourseIdentifier>,
    client: &reqwest::Client,
) -> anyhow::Result<Vec<Activity>> {
    let course_identifiers = course_identifiers.into_iter().collect_vec();
    let query = vec![("type", "course"), ("sem", &*semester)];

    let courses_query = course_identifiers
        .iter()
        .map(|course_identifier| {
            let CourseIdentifier {
                course_code,
                course_term,
            } = course_identifier;

            format!("{},{}", course_code.deref(), course_term)
        })
        .map(|course_identifier| ("id[]".to_string(), course_identifier))
        .collect_vec();

    let res = client
        .get("https://tp.educloud.no/ntnu/timeplan/index.php?type=course")
        .query(&query)
        .query(&courses_query)
        .send()
        .await?;

    let html = res.text().await?;
    let document = Html::parse_document(&html);

    let selector = Selector::parse("script#data-js").unwrap();
    let data = document.select(&selector).next().unwrap().inner_html();

    #[derive(Debug, Deserialize)]
    struct ParsedRoom {
        #[serde(rename = "roomname")]
        pub name: String,

        #[serde(rename = "buildingname")]
        pub building_name: String,

        #[serde(rename = "roomurl")]
        pub url: String,
    }

    impl Into<Room> for ParsedRoom {
        fn into(self) -> Room {
            Room {
                name: self.name,
                building_name: self.building_name,
                url: self.url,
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

    impl Into<StaffMember> for ParsedStaffMember {
        fn into(self) -> StaffMember {
            StaffMember {
                first_name: self.first_name,
                last_name: self.last_name,
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
        pub student_groups: Vec<String>,

        #[serde(rename = "room")]
        pub rooms: Option<Vec<ParsedRoom>>,
    }

    fn convert_activity(
        parsed_activity: ParsedActivity,
        course_identifiers: &Vec<CourseIdentifier>,
    ) -> anyhow::Result<Activity> {
        let course_identifiers = course_identifiers.into_iter().collect_vec();

        fn parse_date_time(input: String) -> anyhow::Result<DateTime<Utc>> {
            let date_time = DateTime::parse_from_str(&input, "%FT%T%#z")?.into();

            Ok(date_time)
        }

        fn vec_into<From: Into<To>, To>(vec: Vec<From>) -> Vec<To> {
            vec.into_iter().map_into().collect()
        }

        let course_code = CourseCode(parsed_activity.course_code);

        let course_identifier = course_identifiers
            .iter()
            .find(|course_identifier| course_identifier.course_code == course_code)
            .unwrap();

        let course_identifier = (**course_identifier).clone();

        let activity = Activity {
            id: parsed_activity.id,
            course_identifier,
            week: parsed_activity.week,
            start: parse_date_time(parsed_activity.start)?,
            end: parse_date_time(parsed_activity.end)?,
            title: parsed_activity.title,
            summary: parsed_activity.summary,
            staff_members: parsed_activity
                .staff_members
                .map(vec_into)
                .unwrap_or_default(),
            student_groups: HashSet::from_iter(parsed_activity.student_groups),
            rooms: parsed_activity.rooms.map(vec_into).unwrap_or_default(),
        };

        Ok(activity)
    }

    let parsed_activities = serde_json::from_str::<Vec<ParsedActivity>>(&data)?;

    let activities = parsed_activities
        .into_iter()
        .map(move |parsed_activity| convert_activity(parsed_activity, &course_identifiers))
        .try_collect()?;

    Ok(activities)
}
