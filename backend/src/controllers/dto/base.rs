use serde::Serialize;
use validator::Validate;

#[derive(Debug, Serialize, Validate)]
pub struct PaginatedResponse<T> {
    pub page: usize,
    pub page_size: usize,
    pub page_elts: i64,
    pub total: i64,
    pub data: Vec<T>,
}
