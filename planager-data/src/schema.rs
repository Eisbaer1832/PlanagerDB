// @generated automatically by Diesel CLI.

diesel::table! {
    classes (year, postFix) {
        year -> Integer,
        postFix -> Nullable<Text>,
    }
}

diesel::table! {
    lessons (subject, teacher, time) {
        subject -> Text,
        teacher -> Text,
        time -> Integer,
        note -> Nullable<Text>,
        canceled -> Nullable<Integer>,
        year -> Integer,
        postFix -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(classes, lessons,);
