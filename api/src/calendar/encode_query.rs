use crate::error::{AppError, AppResult};
use crate::shared_types::CalendarQuery;
use data_encoding::BASE64URL_NOPAD;

pub fn encode_calendar_query(query: &[CalendarQuery]) -> AppResult<String> {
    let query_bytes = rmp_serde::to_vec(query).map_err(|_| AppError::ParsingError)?;
    let encoded_query = BASE64URL_NOPAD.encode(&query_bytes);

    Ok(encoded_query)
}

pub fn decode_calendar_query(query: &str) -> AppResult<Vec<CalendarQuery>> {
    let bytes = BASE64URL_NOPAD
        .decode(query.as_bytes())
        .map_err(|_| AppError::ParsingError)?;

    let calendar_queries =
        rmp_serde::from_slice::<Vec<CalendarQuery>>(&bytes).map_err(|_| AppError::ParsingError)?;

    Ok(calendar_queries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_types::CourseIdentifier;

    #[test]
    fn test_encode_decode() {
        let input = vec![CalendarQuery {
            identifier: CourseIdentifier {
                course_code: "PROG1004".to_owned(),
                semester: "23v".to_owned(),
                course_term: 1,
            },
            student_groups: vec!["BPROG_2".to_owned()],
        }];

        let encoded = encode_calendar_query(&input).unwrap();
        let decoded = decode_calendar_query(&encoded).unwrap();

        assert_eq!(input, decoded);
    }
}
