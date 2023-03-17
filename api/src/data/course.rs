use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Course {
    pub name: String,
    pub amount_of_terms: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseIdentifier {
    pub course_code: String,
    pub course_term: i32,
}
