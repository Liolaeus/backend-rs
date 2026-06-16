use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PaginatedQuery {
    #[validate(range(min = 1))]
    pub page: u8,
    #[validate(range(min = 1, max = 100))]
    pub page_size: u8,
}

#[derive(Debug, Serialize, Validate)]
pub struct PaginatedResponse<T> {
    pub page: u8,
    pub page_size: u8,
    pub page_elts: i64,
    pub total: i64,
    pub data: Vec<T>,
}
