use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::HeaderMap,
    routing::{post, put},
};
use chrono::Utc;
use sha256;
use validator::Validate;

use crate::{
    domain::state::AppState,
    http::{
        dto::{
            GenericAPIResponse,
            users::{AuthUserQuery, UserWrite},
        },
        errors::APIError,
    },
};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/totp/register", put(register))
        .route("/totp/auth", post(authenticate))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserWrite>,
) -> Result<Json<GenericAPIResponse>, APIError> {
    let mut users = state.get_users_map()?;

    if let Err(e) = body.validate() {
        return Err(APIError::BadRequestMsg(e.to_string()));
    }

    if users.contains_key(&body.user.to_string()) {
        return Err(APIError::BadRequestMsg("user already exists".to_string()));
    }

    let encrypted = vec![1, 1, 1, 1, 1, 1];

    users.insert(body.user, encrypted);

    Ok(Json(GenericAPIResponse {
        result: "ok".to_string(),
    }))
}

async fn authenticate(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<AuthUserQuery>,
) -> Result<Json<GenericAPIResponse>, APIError> {
    if let Err(e) = body.validate() {
        return Err(APIError::BadRequestMsg(e.to_string()));
    }
    let users = state.get_users_map()?;

    // extract username from x-user header
    let username = headers
        .get("X-User")
        .ok_or(APIError::BadRequestMsg("missing X-User header".to_string()))?
        .to_str()
        .map_err(|_| APIError::BadRequestMsg("invalid X-User header".to_string()))?
        .to_string();

    // check if queried user exists
    let encrypted = users.get(&username).ok_or(APIError::BadRequest)?;

    // compute 16 1st chars of sha256(secret+time)
    let now_utc = Utc::now().format("%Y%m%d-%H%M").to_string();
    let hash = sha256::digest(format!("{}{}", encrypted[0], now_utc));

    if hash[0..16] == body.password {
        return Ok(Json(GenericAPIResponse {
            result: "ok".to_string(),
        }));
    }

    Err(APIError::Unauthorized)
}
