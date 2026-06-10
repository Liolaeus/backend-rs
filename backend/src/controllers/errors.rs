use axum::{
    Json,
    http::{self, StatusCode},
    response::{IntoResponse, Response},
};

// add other http error codes when necessary.
#[derive(Debug)]
pub enum APIError {
    BadRequest,
    Unauthorized,
    InternalLog(String),
    BadRequestMsg(String),

    JSONMessage(http::StatusCode, String),
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        match self {
            APIError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            APIError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "result": "unauthorized"
                })),
            )
                .into_response(),

            APIError::BadRequestMsg(msg) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "message": msg })),
            )
                .into_response(),

            APIError::InternalLog(message) => {
                // todo: proper logging
                println!("{:?}", message);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }

            APIError::JSONMessage(status, msg) => {
                (status, Json(serde_json::json!({ "result": msg }))).into_response()
            }
        }
    }
}
