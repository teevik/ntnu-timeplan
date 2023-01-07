use chrono::{DateTime, Utc};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub building_name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct StaffMember {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
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
