pub mod base;
pub mod calculator;
pub mod shop;
pub mod users;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct APIResponse {
    pub result: String,
}

// pub fn validate_against_list(
//     fields: &Vec<String>,
//     allowed: &[&str],
// ) -> Result<(), ValidationError> {
//     for field in fields {
//         if !allowed.contains(&field.as_str()) {
//             let mut err = ValidationError::new("invalid field");
//             err.add_param("value".into(), &field);
//             return Err(err);
//         }
//     }
//     Ok(())
// }
