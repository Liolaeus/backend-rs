// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        stock -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(articles, users,);
