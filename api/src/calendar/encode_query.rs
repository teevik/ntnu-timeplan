use crate::shared_types::CalendarQuery;
use data_encoding::BASE64URL_NOPAD;

pub fn encode_calendar_query(query: &[CalendarQuery]) -> anyhow::Result<String> {
    let query_bytes = rmp_serde::to_vec(query)?;
    let encoded_query = BASE64URL_NOPAD.encode(&query_bytes);

    Ok(encoded_query)
}

pub fn decode_calendar_query(query: &str) -> anyhow::Result<Vec<CalendarQuery>> {
    let bytes = BASE64URL_NOPAD.decode(query.as_bytes())?;
    let calendar_queries = rmp_serde::from_slice::<Vec<CalendarQuery>>(&bytes)?;

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
