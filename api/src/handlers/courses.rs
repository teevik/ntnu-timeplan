use crate::app_error::AppError;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

#[utoipa::path(
    get,
    path = "/courses",
    responses(
        (status = 200, body = [Course])
    )
)]
pub async fn courses_handler(state: State<AppState>) -> Result<impl IntoResponse, AppError> {
    let courses_cache = &state.courses_cache;

    let courses = courses_cache.get_or_fetch().await?;
    let courses = (*courses).clone();

    Ok(Json(courses))
}
