use crate::caching::activities_cache::ActivitiesCache;
use crate::caching::courses_cache::CoursesCache;
use crate::caching::semesters_cache::SemestersCache;
use crate::data::course::{CourseCode, CourseIdentifier};
use crate::data::timetable::{generate_timetable, TimetableQuery};
use crate::fetch::courses::fetch_courses;
use crate::fetch::semesters::fetch_semesters;
use crate::graphql_model::{QueryRoot, TimeplanSchema};
use async_graphql::http::GraphiQLSource;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};
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
pub struct AppState {
    pub activities_cache: Arc<ActivitiesCache>,
    pub courses_cache: Arc<CoursesCache>,
    pub semesters_cache: Arc<SemestersCache>,
}

impl AppState {
    pub fn from_context<'a>(context: &'a Context) -> &'a Self {
        let app_state = context.data_unchecked::<AppState>();

        app_state
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let reqwest_client = reqwest::Client::new();

    // // todo: caching elns?
    // let fetched_semesters = fetch_semesters(&reqwest_client).await?;
    // let courses = fetch_courses(&reqwest_client).await?;

    let activities_cache = ActivitiesCache::new(reqwest_client.clone());
    let courses_cache = CoursesCache::new(reqwest_client.clone()).await?;
    let semesters_cache = SemestersCache::new(reqwest_client.clone()).await?;

    let app_state = AppState {
        activities_cache: Arc::new(activities_cache),
        courses_cache: Arc::new(courses_cache),
        semesters_cache: Arc::new(semesters_cache),
    };

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>()?,
        Err(_) => 8080,
    };

    // async fn root(app_state: State<AppState>) -> String {
    //     // let semesters = fetch_semesters(&app_state.reqwest_client).await.unwrap();
    //     // let primary_semester = semesters.primary_semester;
    //
    //     let query = [
    //         TimetableQuery {
    //             course: CourseIdentifier {
    //                 course_code: CourseCode("BMA1020".to_string()),
    //                 course_term: 1,
    //             },
    //             target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
    //         },
    //         TimetableQuery {
    //             course: CourseIdentifier {
    //                 course_code: CourseCode("EXPH0300".to_string()),
    //                 course_term: 1,
    //             },
    //             target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
    //         },
    //         TimetableQuery {
    //             course: CourseIdentifier {
    //                 course_code: CourseCode("PROG1003".to_string()),
    //                 course_term: 1,
    //             },
    //             target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
    //         },
    //         TimetableQuery {
    //             course: CourseIdentifier {
    //                 course_code: CourseCode("PROG1004".to_string()),
    //                 course_term: 1,
    //             },
    //             target_student_groups: HashSet::from(["BPROG_2_".to_string()]),
    //         },
    //     ];
    //
    //     let a = generate_timetable(
    //         &primary_semester,
    //         query,
    //         &app_state.reqwest_client,
    //         &app_state.activities_cache,
    //     )
    //     .await
    //     .unwrap();
    //
    //     a.to_string()
    // }

    async fn graphiql() -> impl IntoResponse {
        Html(GraphiQLSource::build().endpoint("/").finish())
    }

    let schema = TimeplanSchema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(app_state.clone())
        .finish();

    async fn graphql_handler(
        schema: Extension<TimeplanSchema>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }

    // /timetable.ics?
    let router = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
