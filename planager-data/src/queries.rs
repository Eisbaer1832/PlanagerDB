use diesel::dsl::count;
use diesel::prelude::*;
use crate::schema::lessons;
use crate::schema::lessons::{canceled, subject};

pub fn fetch_subject_cancellation(conn: &mut SqliteConnection) -> Vec<(String, i64)> {

    let subjects = lessons::table
        .group_by(subject)
        .select((subject,count(canceled)))
        .load::<(String, i64)>(conn)
        .expect("Can't load data");

    
    subjects
}