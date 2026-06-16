use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::controllers::dto::base::{PaginatedQuery, PaginatedResponse};

pub type PaginatedUsers = PaginatedResponse<UserRead>;

// match 4-16 long alphanumerical uersernames
static USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{4,16}$").unwrap());
// match 8-64 long alphanumerical passwords
static PASSWORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{8,64}$").unwrap());
// very basic email regex
static EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\w+(?:.\w+)*@\w+\.\w+$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct UserWrite {
    #[validate(regex(path=*USERNAME))]
    pub name: String,
    #[validate(regex(path=*EMAIL))]
    pub email: String,
    #[validate(regex(path=*PASSWORD))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRead {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthUserQuery {
    #[validate(regex(path=*PASSWORD))]
    pub password: String,
}

// fn validate_users_sort_by(sort_by: &Vec<String>) -> Result<(), ValidationError> {
//     validate_against_list(sort_by, &["name", "email"])
// }

#[derive(Debug, Deserialize, Validate)]
pub struct PaginatedUserQuery {
    #[serde(flatten)]
    pub base: PaginatedQuery,
    // #[validate(custom(function = "validate_users_sort_by"))]
    // pub sort_by: Vec<String>,
}
