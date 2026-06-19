use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PaginatedQuery {
    #[validate(range(min = 1))]
    pub page: usize,
    #[validate(range(min = 1, max = 100))]
    pub page_size: usize,
}
