use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

// match 4-16 long alphanumerical uersernames
static USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{4,16}$").unwrap());
// match 8-64 long alphanumerical passwords
static PASSWORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{8,64}$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct UserWrite {
    #[validate(regex(path=*USERNAME))]
    pub user: String,
    #[validate(regex(path=*PASSWORD))]
    pub secret: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthUserQuery {
    #[validate(regex(path=*PASSWORD))]
    pub password: String,
}
