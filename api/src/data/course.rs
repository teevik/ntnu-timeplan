use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseCode(pub String);

#[derive(Debug, Clone)]
pub struct Course {
    pub name: String,
    pub amount_of_terms: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseIdentifier {
    pub course_code: CourseCode,
    pub course_term: i32,
}
