use crate::schema::lessons::{canceled, postFix, subjectId, year};
use crate::schema::lessons::note;
use crate::schema::lessons::time;
use crate::schema::lessons::teacher;
use crate::schema::lessons::subject;
use diesel::ExpressionMethods;
use diesel::dsl::insert_or_ignore_into;
use diesel::{RunQueryDsl, SqliteConnection};
use crate::sanitize::sanitize_subject;
use crate::{Class, Lesson};
use crate::schema::lessons::dsl::lessons;

pub fn insert_lesson(lesson: &Lesson, class: &Class, conn: &mut SqliteConnection) {
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
pub fn populate_database(cs: Vec<Class>, conn: &mut SqliteConnection) {

    for class in cs {
        for lesson in &class.lessons {
            insert_lesson(lesson, &class, conn);
        }
    }
}