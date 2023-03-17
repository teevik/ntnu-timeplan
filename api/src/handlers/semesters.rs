use crate::app_error::AppError;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

pub async fn semesters_handler(state: State<AppState>) -> Result<impl IntoResponse, AppError> {
    let semester_cache = &state.semesters_cache;

    let semesters = semester_cache.get_or_fetch().await?;
    let semesters = (*semesters).clone();

    Ok(Json(semesters))
}
