pub mod calculator;
pub mod shop;
pub mod users;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct APIResponse {
    pub result: String,
}
