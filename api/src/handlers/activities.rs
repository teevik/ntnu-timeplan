use crate::app_error::AppError;
use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use ntnu_timeplan_shared::CourseIdentifier;

pub async fn activities_handler(
    state: State<AppState>,
    course_identifier: Query<CourseIdentifier>,
) -> Result<impl IntoResponse, AppError> {
    let activities_cache = &state.activities_cache;

    let activities = activities_cache.get_or_fetch(course_identifier.0).await?;
    let activities = (*activities).clone();

    Ok(Json(activities))
}
