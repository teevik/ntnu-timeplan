use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Semester(pub String);
