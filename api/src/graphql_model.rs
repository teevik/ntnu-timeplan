use crate::data::activities_for_courses::ActivitiesForCourse;
use crate::data::activity::{Activity, Room, StaffMember};
use crate::data::course::{Course, CourseCode, CourseIdentifier};
use crate::data::semester::Semester;
use crate::fetch::semesters::FetchedSemester;
use crate::AppState;
use async_graphql::{Context, EmptyMutation, EmptySubscription, InputObject, Object, Schema};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use std::sync::Arc;

pub type TimeplanSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct CourseModel {
    course_code: CourseCode,
    course: Course,
}

#[Object]
impl CourseModel {
    async fn id(&self) -> &str {
        &*self.course_code
    }

    async fn name(&self) -> &str {
        &*self.course.name
    }

    async fn amount_of_terms(&self) -> i32 {
        self.course.amount_of_terms
    }
}

pub struct SemesterModel(FetchedSemester);

#[Object]
impl SemesterModel {
    async fn id(&self) -> &str {
        &*self.0.semester
    }

    async fn name(&self) -> &str {
        &*self.0.semester
    }
}

pub struct StaffMemberModel(StaffMember);

#[Object]
impl StaffMemberModel {
    async fn first_name(&self) -> &str {
        &*self.0.first_name
    }

    async fn last_name(&self) -> &str {
        &*self.0.last_name
    }
}

pub struct RoomModel(Room);

#[Object]
impl RoomModel {
    async fn name(&self) -> &str {
        &*self.0.name
    }

    async fn building_name(&self) -> &str {
        &*self.0.building_name
    }

    async fn url(&self) -> &str {
        &*self.0.url
    }
}

pub struct ActivityModel<'a>(&'a Activity);

#[Object]
impl<'a> ActivityModel<'a> {
    async fn id(&self) -> &str {
        &*self.0.id
    }

    async fn course_code(&self) -> &str {
        &*self.0.course_identifier.course_code
    }

    async fn week(&self) -> i32 {
        self.0.week
    }

    async fn start(&self) -> &DateTime<Utc> {
        &self.0.start
    }

    async fn end(&self) -> &DateTime<Utc> {
        &self.0.end
    }

    async fn title(&self) -> &str {
        &*self.0.title
    }

    async fn summary(&self) -> &str {
        &*self.0.summary
    }

    async fn staff_members(&self) -> Vec<StaffMemberModel> {
        self.0
            .staff_members
            .iter()
            .cloned()
            .map(|staff_member| StaffMemberModel(staff_member))
            .collect()
    }

    async fn student_groups(&self) -> Vec<String> {
        self.0.student_groups.iter().cloned().collect()
    }

    async fn rooms(&self) -> Vec<RoomModel> {
        self.0
            .rooms
            .iter()
            .cloned()
            .map(|room| RoomModel(room))
            .collect()
    }
}

pub struct ActivitiesForCourseModel(Arc<ActivitiesForCourse>);

#[Object]
impl ActivitiesForCourseModel {
    async fn activities<'a>(&'a self) -> Vec<ActivityModel<'a>> {
        self.0
            .activities
            .iter()
            .map(|activity| ActivityModel(activity))
            .collect()
    }

    async fn student_groups(&self) -> Vec<String> {
        self.0.all_student_groups.iter().cloned().collect()
    }
}

pub struct QueryRoot;

#[derive(InputObject, Clone)]
struct CourseIdentifierModel {
    course_code: String,
    course_term: i32,
}

impl From<CourseIdentifierModel> for CourseIdentifier {
    fn from(model: CourseIdentifierModel) -> Self {
        CourseIdentifier {
            course_code: CourseCode(model.course_code),
            course_term: model.course_term,
        }
    }
}

#[Object]
impl QueryRoot {
    async fn courses<'ctx>(&self, ctx: &Context<'ctx>) -> anyhow::Result<Vec<CourseModel>> {
        let app_state = AppState::from_context(ctx);

        let courses = app_state.courses_cache.get_or_fetch().await?;

        let course_models = courses
            .iter()
            .map(|(course_code, course)| CourseModel {
                course_code: course_code.clone(),
                course: course.clone(),
            })
            .collect();

        Ok(course_models)
    }

    async fn semesters<'ctx>(&self, ctx: &Context<'ctx>) -> anyhow::Result<Vec<SemesterModel>> {
        let app_state = AppState::from_context(ctx);

        let semesters = app_state.semesters_cache.get_or_fetch().await?;

        let semesters = semesters
            .semesters
            .iter()
            .map(|semester| SemesterModel(semester.clone()))
            .collect();

        Ok(semesters)
    }

    async fn current_semester<'ctx>(&self, ctx: &Context<'ctx>) -> anyhow::Result<SemesterModel> {
        let app_state = AppState::from_context(ctx);

        let semesters = app_state.semesters_cache.get_or_fetch().await?;
        let current_semester = SemesterModel(semesters.current_semester.clone());

        Ok(current_semester)
    }

    async fn activities<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        semester: String,
        courses: Vec<CourseIdentifierModel>,
    ) -> anyhow::Result<Vec<ActivitiesForCourseModel>> {
        let app_state = AppState::from_context(ctx);

        let a = app_state
            .activities_cache
            .get_or_fetch_activities(&Semester(semester), courses.into_iter().map_into())
            .await?;

        let pog = a
            .iter()
            .map(|(_, b)| ActivitiesForCourseModel(b.clone()))
            .collect();

        Ok(pog)
    }
}
