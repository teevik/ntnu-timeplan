#![feature(anonymous_lifetime_in_impl_trait)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Room {
    pub name: String,
    pub building_name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StaffMember {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub name: String,
    pub amount_of_terms: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseIdentifier {
    pub course_code: String,
    pub course_term: i32,
    pub semester: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Semester {
    pub semester: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemestersWithCurrent {
    pub semesters: Vec<Semester>,
    pub current_semester: Semester,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct A {}
