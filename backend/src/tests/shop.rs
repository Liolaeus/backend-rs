#![allow(unused)]

use std::collections::HashMap;

use axum::http::StatusCode;
use rstest::rstest;

use crate::{
    domain::state::{Stock, destock},
    controllers::errors::APIError,
};

#[rstest]
#[case(HashMap::from([(1, 10)]), HashMap::from([(1, 3)]), Ok(HashMap::from([(1, 7)])))]
#[case(
	HashMap::from([(1, 10), (2, 8)]),
	HashMap::from([(1, 3), (2, 5)]),
	Ok(HashMap::from([(1, 7), (2, 3)]))
)]
#[case(
	HashMap::from([(1, 10), (2, 8)]),
	HashMap::from([(1, 10), (2, 8)]),
	Ok(HashMap::from([(1, 0), (2, 0)]))
)]
#[case(HashMap::from([(42, 0)]), HashMap::from([(42, 0)]), Ok(HashMap::from([(42, 0)])))]
#[case(
	HashMap::from([(1, 1)]),
	HashMap::from([(1, 2)]),
	Err(APIError::JSONMessage(StatusCode::BAD_REQUEST, "oos".to_string()))
)]
#[case(
	HashMap::from([(1, 1)]),
	HashMap::from([(999, 1)]),
	Err(APIError::BadRequestMsg("product 999 does not exist".to_string()))
)]
fn test_destock(
    #[case] mut stock: Stock,
    #[case] basket: Stock,
    #[case] expected: Result<Stock, APIError>,
) {
    match (destock(&mut stock, &basket), expected) {
        (Ok(()), Ok(expected_stock)) => assert_eq!(stock, expected_stock),
        (
            Err(APIError::JSONMessage(actual_code, actual_msg)),
            Err(APIError::JSONMessage(expected_code, expected_msg)),
        ) => {
            assert_eq!(actual_code, expected_code);
            assert_eq!(actual_msg, expected_msg);
        }
        (Err(APIError::BadRequestMsg(actual_msg)), Err(APIError::BadRequestMsg(expected_msg))) => {
            assert_eq!(actual_msg, expected_msg);
        }
        (actual, expected) => panic!(
            "unexpected result; got: {:?}, expected: {:?}",
            actual, expected
        ),
    }
}
