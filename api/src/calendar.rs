// use crate::data::timetable::{generate_timetable, TimetableQuery};
// use crate::AppState;
// use axum::extract::{RawQuery, State};
// use axum::response::IntoResponse;
// use serde::{Deserialize, Serialize};
//
// // #[derive(Serialize, Deserialize, Debug)]
// // pub struct CourseQuery {
// //     pub course_code: String,
// //     pub course_term: i32,
// //     pub student_groups: HashSet<String>,
// // }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CalendarQuery {
//     pub semester: String,
//     pub timetable_queries: Vec<TimetableQuery>,
// }
//
// pub async fn calendar(app_state: State<AppState>, raw_query: RawQuery) -> impl IntoResponse {
//     let raq_query = (raw_query.0.as_ref()).unwrap();
//
//     let calendar_query: CalendarQuery = serde_qs::from_str(&raq_query).unwrap();
//
//     let calendar = generate_timetable(
//         &calendar_query.semester,
//         calendar_query.timetable_queries,
//         &app_state.activities_cache,
//     )
//     .await
//     .unwrap();
//
//     calendar.to_string()
//
//     // serde_qs::to_string(&CalendarQuery {
//     //     semester: "22v".to_string(),
//     //     timetable_queries: vec![TimetableQuery {
//     //         course: CourseIdentifier {
//     //             course_code: "PROG1003".to_owned(),
//     //             course_term: 1,
//     //         },
//     //         target_student_groups: HashSet::from(["BPROG_2_".to_owned()]),
//     //     }],
//     // })
//     // .unwrap()
// }
