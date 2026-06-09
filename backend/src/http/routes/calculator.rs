use std::sync::Arc;

use axum::{Router, extract::Query, routing::get};

use crate::{
    domain::{
        calculator::{ExpError, evaluate_expression},
        state::AppState,
    },
    http::dto::calculator::CalcQuery,
};

pub fn calc_routes() -> Router<Arc<AppState>> {
    Router::new().route("/calculatrice", get(calculate))
}

pub async fn calculate(Query(params): Query<CalcQuery>) -> Result<String, ExpError> {
    match evaluate_expression(&params.expr) {
        Ok(num) => Ok(num.to_string()),
        Err(err) => Err(err),
    }
}
