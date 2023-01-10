use crate::data::activity::Activity;
use crate::data::course::CourseIdentifier;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ActivitiesForCourse {
    pub all_student_groups: HashSet<String>,
    pub activities: Vec<Activity>,
}

pub fn get_activities_for_courses(
    activities: Vec<Activity>,
) -> HashMap<CourseIdentifier, ActivitiesForCourse> {
    let mut activities_for_courses = HashMap::<CourseIdentifier, ActivitiesForCourse>::new();

    for activity in activities {
        let empty_activities_for_course = || ActivitiesForCourse {
            all_student_groups: Default::default(),
            activities: Default::default(),
        };

        let activities_for_course = activities_for_courses
            .entry(activity.course_identifier.clone())
            .or_insert_with(|| empty_activities_for_course());

        activities_for_course
            .all_student_groups
            .extend(activity.student_groups.iter().cloned());

        activities_for_course.activities.push(activity);
    }

    activities_for_courses
}
