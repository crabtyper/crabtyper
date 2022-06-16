use diesel::sql_types;

table! {
    languages (id) {
        id -> Text,
        name -> Text,
    }
}

table! {
    snippets (id) {
        id -> Text,
        code -> Text,
        language_id -> Text,
    }
}

joinable!(snippets -> languages (language_id));

allow_tables_to_appear_in_same_query!(languages, snippets,);

no_arg_sql_function!(
    random,
    sql_types::Integer,
    "Represents the SQL RANDOM() function"
);
