use axum::extract::State;
use deadpool_diesel::postgres::Pool;
use diesel::{
    RunQueryDsl, Selectable, SelectableHelper, deserialize::Queryable, prelude::Insertable,
};
use serde::{Deserialize, Serialize};

use crate::{controllers::errors::APIError, schema::users};

#[derive(Serialize, Queryable, Selectable, Deserialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDB {
    pub name: String,
    pub email: String,
}

pub async fn create_user(pool: &Pool, new_user: UserDB) -> Result<UserDB, APIError> {
    let db = pool.get().await.unwrap();

    let res = db
        .interact(|db| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(UserDB::as_returning())
                .get_result(db)
        })
        .await
        .unwrap()
        .unwrap();

    Ok(res)
}
