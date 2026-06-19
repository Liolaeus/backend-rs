use anyhow::Result;
use deadpool_diesel::{InteractError, postgres::Pool};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, Selectable, SelectableHelper,
    deserialize::Queryable, prelude::Insertable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    controllers::{
        dto::users::UserRead,
        queries::PaginatedQuery,
    },
    schema::users as schema_users,
};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = schema_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDB {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl UserDB {
    pub fn to_user_read(&self) -> UserRead {
        UserRead {
            uuid: self.uuid,
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

pub async fn create_user(pool: &Pool, new_user: UserDB) -> Result<UserDB, DBError> {
    let db = pool.get().await.unwrap();

    let res = db
        .interact(|db| {
            diesel::insert_into(schema_users::table)
                .values(new_user)
                .returning(UserDB::as_returning())
                .get_result(db)
        })
        .await??;

    Ok(res)
}

pub async fn get_users(
    pool: &Pool,
    query: &PaginatedQuery,
) -> Result<(Vec<UserDB>, i64, i64), DBError> {
    let db = pool.get().await.unwrap();

    let offset = (query.page - 1) as i64 * query.page_size as i64;
    let limit = query.page_size as i64;

    let (users, total, page_elts) = db
        .interact(move |db| {
            db.transaction::<_, diesel::result::Error, _>(|db| {
                let total = schema_users::table.count().get_result(db)?;

                let res = schema_users::table
                    .offset(offset)
                    .limit(limit)
                    .order(schema_users::name)
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

pub async fn delete_user(pool: &Pool, uid: Uuid) -> Result<usize, DBError> {
    let db = pool.get().await.unwrap();

    let count = db
        .interact(move |db| {
            diesel::delete(schema_users::table.filter(schema_users::uuid.eq(uid))).execute(db)
        })
        .await??;

    Ok(count)
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
