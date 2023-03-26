use crate::app_error::AppError;
use crate::calendar_queries::encode_calendar_query;
use crate::shared_types::CalendarQuery;
use axum::response::IntoResponse;
use axum::Json;

#[utoipa::path(
    post,
    path = "/encode-calendar-query",
    request_body = [CalendarQuery],
    responses(
        (status = 200, body = String)
    )
)]
pub async fn encode_calendar_query_handler(
    calendar_queries: Json<Vec<CalendarQuery>>,
) -> Result<impl IntoResponse, AppError> {
    let encoded_query = encode_calendar_query(&calendar_queries)?;

    Ok(encoded_query)
}
