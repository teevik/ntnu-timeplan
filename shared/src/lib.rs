#![feature(anonymous_lifetime_in_impl_trait)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use ts_rs::TS;

#[derive(TS, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Room {
    pub name: String,
    pub building_name: String,
    pub url: String,
}

#[derive(TS, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StaffMember {
    pub first_name: String,
    pub last_name: String,
}

#[derive(TS, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Activity {
    pub id: String,
    pub course_code: String,
    pub week: i32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub title: String,
    pub summary: String,
    pub staff_members: Vec<StaffMember>,
    pub student_groups: HashSet<String>,
    pub rooms: Vec<Room>,
}

pub fn all_student_groups(activities: impl IntoIterator<Item = &Activity>) -> HashSet<String> {
    let activities = activities.into_iter();

    let mut all_student_groups = HashSet::new();

    for activity in activities {
        all_student_groups.extend(activity.student_groups.iter().cloned());
    }

    all_student_groups
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Course {
    pub name: String,
    pub amount_of_terms: i32,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CourseIdentifier {
    pub course_code: String,
    pub course_term: i32,
    pub semester: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Semester {
    pub name: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SemestersWithCurrent {
    pub semesters: HashMap<String, Semester>,
    pub current_semester: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CalendarQuery {
    pub identifier: CourseIdentifier,
    pub student_groups: HashSet<String>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CalendarQueries {
    pub queries: Vec<CalendarQuery>,
}

impl CalendarQueries {
    pub fn to_query_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_query_string(string: &str) -> serde_json::Result<Self> {
        serde_json::from_str(string)
    }
}