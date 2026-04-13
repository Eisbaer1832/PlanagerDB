mod schema;
pub mod database;

#[derive(Default)]
pub struct Subject {
    pub name: String,
    pub teacher: String
}


#[derive(Default)]
pub struct Lesson {
    pub subject: Subject,
    pub time: i32,
    pub note: String,
    pub canceled:bool,
}

#[derive(Default)]
pub struct Class {
    pub year : i32,
    pub post_fix: String,
    pub subjects : Vec<Subject>,
    pub lessons: Vec<Lesson>,
}