// @generated automatically by Diesel CLI.

diesel::table! {
    articles (uuid) {
        uuid -> Uuid,
        stock -> Int4,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password_hash -> VarChar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(articles, users,);
