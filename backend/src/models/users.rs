use anyhow::Result;
use deadpool_diesel::{InteractError, postgres::Pool};
use diesel::{
    ExpressionMethods, RunQueryDsl, Selectable, SelectableHelper,
    deserialize::Queryable,
    prelude::Insertable,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use serde::{Deserialize, Serialize};

use crate::{controllers::dto::users::UserRead, schema::users};

#[derive(Serialize, Queryable, Selectable, Deserialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDB {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl UserDB {
    pub fn to_user_read(&self) -> UserRead {
        UserRead {
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

pub async fn create_user(pool: &Pool, new_user: UserDB) -> Result<UserDB, DBError> {
    let db = pool.get().await.unwrap();

    let res = db
        .interact(|db| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(UserDB::as_returning())
                .get_result(db)
        })
        .await??;

    Ok(res)
}

pub async fn read_user(pool: &Pool, email: String) -> Result<UserDB, DBError> {
    let db = pool.get().await.unwrap();

    let res = db
        .interact(|db| {
            users::table
                .filter(users::email.eq(email))
                .select(UserDB::as_select())
                .get_result(db)
        })
        .await??;

    Ok(res)
}

#[derive(Debug, thiserror::Error)]
pub enum DBError {
    // #[error("database interaction failed: {source}")]
    // Interact {
    //     #[from]
    //     source: InteractError,
    // },
    #[error(transparent)]
    Interact(#[from] InteractError),

    // #[error("database error: {source}")]
    // Diesel {
    //     #[from]
    //     source: diesel::result::Error,
    // },
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
}
