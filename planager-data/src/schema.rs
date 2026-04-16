// @generated automatically by Diesel CLI.

diesel::table! {
    lessons (rowid) {
        rowid -> Integer,
        subject -> Text,
        subjectId -> Text,
        teacher -> Text,
        time -> Integer,
        note -> Nullable<Text>,
        canceled -> Nullable<Integer>,
        year -> Integer,
        postFix -> Text,
        date -> Nullable<Text>,
    }
}
