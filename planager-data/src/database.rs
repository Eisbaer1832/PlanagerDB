use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::dsl::{insert_or_ignore_into};
use crate::{schema, Class, Lesson};
use crate::sanitize::sanitize_subject;
use crate::schema::classes::dsl::classes;
use crate::schema::lessons::dsl::lessons;
use crate::schema::lessons::{canceled, note, postFix, subject, subjectId, teacher, time, year};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}

pub fn insert_class(class: &Class) {
    let conn = &mut establish_connection();

    insert_or_ignore_into(classes)
        .values((
            schema::classes::year.eq(class.year),
            schema::classes::postFix.eq(&class.post_fix)
        ))
        .execute(conn).expect("TODO: panic message");
        
}


pub fn insert_lesson(lesson: &Lesson, class: &Class) {
    let conn = &mut establish_connection();
    let canceled_sql = if lesson.canceled { 1 } else { 0 };
    
    insert_or_ignore_into(lessons)
        .values((
            subject.eq(sanitize_subject(&lesson.subject.name)),
            subjectId.eq(&lesson.subject.name),
            teacher.eq(&lesson.subject.teacher),
            time.eq(&lesson.time),
            note.eq(&lesson.note),
            canceled.eq(canceled_sql),
            year.eq(&class.year),
            postFix.eq(&class.post_fix)
        ))
        .execute(conn).expect("TODO: panic message");
}