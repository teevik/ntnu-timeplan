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
