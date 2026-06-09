pub mod calculator;
pub mod shop;
pub mod users;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GenericAPIResponse {
    pub result: String,
}
