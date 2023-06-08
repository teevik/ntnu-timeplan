use std::{collections::HashMap, sync::Arc};

use crate::{
    calendar::encode_query::encode_calendar_query,
    shared_types::{Activity, CalendarQuery, Course, CourseIdentifier, SemestersWithCurrent},
    AppState,
};
use poem::web::Data;
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    OpenApi,
};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(operation_id = "getSemesters", path = "/semesters", method = "get")]
    async fn semesters(
        &self,
        app_state: Data<&AppState>,
    ) -> poem::Result<Json<SemestersWithCurrent>> {
        let semester_cache = &app_state.semesters_cache;

        let semesters = semester_cache.get_or_fetch().await?;
        let semesters = (*semesters).clone();

        Ok(Json(semesters))
    }

    #[oai(operation_id = "getCourses", path = "/courses", method = "get")]
    async fn courses(
        &self,
        app_state: Data<&AppState>,
    ) -> poem::Result<Json<Arc<HashMap<String, Course>>>> {
        let courses_cache = &app_state.courses_cache;

        let courses = courses_cache.get_or_fetch().await?;

        Ok(Json(courses))
    }

    #[oai(operation_id = "getActivities", path = "/activities", method = "get")]
    async fn activities(
        &self,
        app_state: Data<&AppState>,
        #[oai(name = "courseCode")] course_code: Query<String>,
        #[oai(name = "courseTerm")] course_term: Query<i32>,
        semester: Query<String>,
    ) -> poem::Result<Json<Arc<Vec<Activity>>>> {
        let activities_cache = &app_state.activities_cache;

        let activities = activities_cache
            .get_or_fetch(CourseIdentifier {
                course_code: course_code.0,
                course_term: course_term.0,
                semester: semester.0,
            })
            .await?;

        Ok(Json(activities))
    }

    #[oai(
        operation_id = "getEncodedCalendarQuery",
        path = "/encode-calendar-query",
        method = "post"
    )]
    async fn encode_calendar_query(
        &self,
        calendar_queries: Json<Vec<CalendarQuery>>,
    ) -> poem::Result<PlainText<String>> {
        let encoded_query = encode_calendar_query(&calendar_queries)?;

        Ok(PlainText(encoded_query))
    }
}
