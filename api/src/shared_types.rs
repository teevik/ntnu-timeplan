use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Object, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct Room {
    pub name: String,
    pub building_name: String,
    pub url: String,
}

#[derive(Object, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct StaffMember {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Object, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    pub course_code: String,
    pub week: i32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub title: String,
    pub summary: String,
    pub staff_members: Vec<StaffMember>,
    pub student_groups: Vec<String>,
    pub rooms: Vec<Room>,
}

#[derive(Object, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct Course {
    pub name: String,
    pub amount_of_terms: i32,
}

#[derive(Object, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[oai(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct CourseIdentifier {
    pub course_code: String,
    pub course_term: i32,
    pub semester: String,
}

#[derive(Object, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct Semester {
    pub name: String,
}
#[derive(Object, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct SemestersWithCurrent {
    pub semesters: HashMap<String, Semester>,
    pub current_semester: String,
}

#[derive(Object, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[oai(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct CalendarQuery {
    pub identifier: CourseIdentifier,
    pub student_groups: Vec<String>,
}
