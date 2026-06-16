use anyhow::Result;
use deadpool_diesel::{InteractError, postgres::Pool};
use diesel::{
    Connection, QueryDsl, RunQueryDsl, Selectable, SelectableHelper,
    deserialize::Queryable, prelude::Insertable,
};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::dto::users::{PaginatedUserQuery, UserRead},
    schema::users,
};

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

pub async fn get_users(
    pool: &Pool,
    query: &PaginatedUserQuery,
) -> Result<(Vec<UserDB>, i64, i64), DBError> {
    let db = pool.get().await.unwrap();

    let offset = (query.base.page - 1) as i64 * query.base.page_size as i64;
    let limit = query.base.page_size as i64;

    let (users, total, page_elts) = db
        .interact(move |db| {
            db.transaction::<_, diesel::result::Error, _>(|db| {
                let total = users::table.count().get_result(db)?;

                let res = users::table
                    .offset(offset)
                    .limit(limit)
                    .select(UserDB::as_select())
                    .get_results(db);

                match res {
                    Ok(users) => {
                        let page_elts = users.len() as i64;
                        Ok((users, total, page_elts))
                    }
                    Err(e) => Err(e),
                }
            })
        })
        .await??;

    Ok((users, total, page_elts))
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
