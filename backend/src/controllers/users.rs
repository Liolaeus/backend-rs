use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError};
use uuid::Uuid;
use validator::Validate;

use crate::{
    controllers::{
        dto::users::{PaginatedUsers, UserRead, UserWrite},
        errors::APIError,
        queries::PaginatedQuery,
    },
    domain::state::AppState,
    models::users::{DBError, UserDB, create_user, delete_user, get_users},
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_users))
        .route("/users", get(read_users))
        .route("/users/{uuid}", delete(delete_users))
}

async fn create_users(
    State(state): State<AppState>,
    Json(body): Json<UserWrite>,
) -> Result<(StatusCode, Json<UserRead>), APIError> {
    if let Err(e) = body.validate() {
        return Err(APIError::BadRequestMsg(e.to_string()));
    }

    let res = create_user(
        &state.db,
        UserDB {
            uuid: uuid::Uuid::new_v4(),
            name: body.name,
            email: body.email,
            password_hash: hash_password(body.password),
        },
    )
    .await;

    match res {
        Ok(user_db) => Ok((StatusCode::CREATED, Json(user_db.to_user_read()))),

        Err(DBError::Diesel(DatabaseError(UniqueViolation, _))) => {
            Err(APIError::BadRequestMsg("email taken".to_string()))
        }

        Err(err) => Err(APIError::InternalLog(err.to_string())),
    }
}

async fn read_users(
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery>,
) -> Result<(StatusCode, Json<PaginatedUsers>), APIError> {
    if let Err(e) = query.validate() {
        return Err(APIError::BadRequestMsg(e.to_string()));
    }

    let res = get_users(&state.db, &query).await;

    match res {
        Ok((users_db, total, page_elts)) => Ok((
            StatusCode::OK,
            Json(PaginatedUsers {
                page: query.page,
                page_size: query.page_size,
                total,
                page_elts,
                data: users_db.into_iter().map(|u| u.to_user_read()).collect(),
            }),
        )),

        Err(err) => Err(APIError::InternalLog(err.to_string())),
    }
}

async fn delete_users(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, APIError> {
    let res = delete_user(&state.db, uuid).await;

    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(APIError::InternalLog(err.to_string())),
    }
}

fn hash_password(clear: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(clear.as_bytes(), &salt)
        .expect("password hash failed");

    hash.to_string()
}

// async fn authenticate(
//     State(_state): State<AppState>,
//     _headers: HeaderMap,
//     Json(_body): Json<au>,
// ) -> Result<Json<APIResponse>, APIError> {
// if let Err(e) = body.validate() {
//     return Err(APIError::BadRequestMsg(e.to_string()));
// }
// let users = state.get_users_map()?;

// // extract username from x-user header
// let username = headers
//     .get("X-User")
//     .ok_or(APIError::BadRequestMsg("missing X-User header".to_string()))?
//     .to_str()
//     .map_err(|_| APIError::BadRequestMsg("invalid X-User header".to_string()))?
//     .to_string();

// // check if queried user exists
// let encrypted = users.get(&username).ok_or(APIError::BadRequest)?;

// // compute 16 1st chars of sha256(secret+time)
// let now_utc = Utc::now().format("%Y%m%d-%H%M").to_string();
// let hash = sha256::digest(format!("{}{}", encrypted[0], now_utc));

// if hash[0..16] == body.password {
//     return Ok(Json(GenericAPIResponse {
//         result: "ok".to_string(),
//     }));
// }

//     Err(APIError::Unauthorized)
// }
