use crate::shared_types::CalendarQuery;
use data_encoding::BASE64URL_NOPAD;

pub fn encode_calendar_query(query: &[CalendarQuery]) -> color_eyre::Result<String> {
    let query_bytes = rmp_serde::to_vec(query)?;
    let encoded_query = BASE64URL_NOPAD.encode(&query_bytes);

    Ok(encoded_query)
}

pub fn decode_calendar_query(query: &str) -> color_eyre::Result<Vec<CalendarQuery>> {
    let bytes = BASE64URL_NOPAD.decode(query.as_bytes())?;
    let calendar_queries = rmp_serde::from_slice::<Vec<CalendarQuery>>(&bytes)?;

    Ok(calendar_queries)
}
