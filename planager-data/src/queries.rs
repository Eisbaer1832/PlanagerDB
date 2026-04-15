use diesel::dsl::count;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::schema::lessons;
use crate::schema::lessons::{canceled, subject};

pub fn fetch_subject_cancellation() -> Vec<(String, i64)> {
    let mut conn = establish_connection();

    let mut subjects = lessons::table
        .group_by(subject)
        .select((subject,count(canceled)))
        .load::<(String, i64)>(&mut conn)
        .expect("Can't load data");

    
    subjects
}