use crate::app_error::AppError;
use crate::shared_types::CourseIdentifier;
use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;

#[utoipa::path(
    get,
    path = "/activities",
    params(CourseIdentifier),
    responses(
        (status = 200, body = [Activity])
    )
)]
pub async fn activities_handler(
    state: State<AppState>,
    course_identifier: Query<CourseIdentifier>,
) -> Result<impl IntoResponse, AppError> {
    let activities_cache = &state.activities_cache;

    let activities = activities_cache.get_or_fetch(course_identifier.0).await?;
    let activities = (*activities).clone();

    Ok(Json(activities))
}
