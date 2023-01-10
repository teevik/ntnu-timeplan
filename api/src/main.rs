use crate::caching::activities_cache::ActivitiesCache;
use crate::data::course::{CourseCode, CourseIdentifier};
use crate::data::timetable::{generate_timetable, TimetableQuery};
use crate::fetch::courses::fetch_courses;
use crate::fetch::semesters::fetch_semesters;
use async_graphql::http::GraphiQLSource;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::collections::HashSet;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

mod caching;
mod data;
mod fetch;
mod graphql_model;

#[derive(Clone)]
struct AppState {
    pub activities_cache: Arc<ActivitiesCache>,
    pub reqwest_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let reqwest_client = reqwest::Client::new();

    // todo: caching elns?
    let fetched_semesters = fetch_semesters(&reqwest_client).await?;
    let courses = fetch_courses(&reqwest_client).await?;

    let activities_cache = ActivitiesCache::new();
    let app_state = AppState {
        activities_cache: Arc::new(activities_cache),
        reqwest_client,
    };

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>()?,
        Err(_) => 8080,
    };

    async fn root(app_state: State<AppState>) -> String {
        let semesters = fetch_semesters(&app_state.reqwest_client).await.unwrap();
        let primary_semester = semesters.primary_semester;

        let query = [
            TimetableQuery {
                course: CourseIdentifier {
                    course_code: CourseCode("BMA1020".to_string()),
                    course_term: 1,
                },
                target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
            },
            TimetableQuery {
                course: CourseIdentifier {
                    course_code: CourseCode("EXPH0300".to_string()),
                    course_term: 1,
                },
                target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
            },
            TimetableQuery {
                course: CourseIdentifier {
                    course_code: CourseCode("PROG1003".to_string()),
                    course_term: 1,
                },
                target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
            },
            TimetableQuery {
                course: CourseIdentifier {
                    course_code: CourseCode("PROG1004".to_string()),
                    course_term: 1,
                },
                target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
            },
        ];

        let a = generate_timetable(
            &primary_semester,
            query,
            &app_state.reqwest_client,
            &app_state.activities_cache,
        )
        .await
        .unwrap();

        a.to_string()
    }

    async fn graphiql() -> impl IntoResponse {
        Html(GraphiQLSource::build().endpoint("/").finish())
    }

    // /
    // /timetable.ics?
    let router = Router::new()
        .route("/", get(graphiql))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
