use derive_more::Constructor;

#[derive(Debug)]
pub struct Course {
    pub name: String,
    pub amount_of_terms: i32,
}

#[derive(Debug, Constructor)]
pub struct CourseIdentifier<'a> {
    pub course_code: &'a str,
    pub course_term: i32,
}
