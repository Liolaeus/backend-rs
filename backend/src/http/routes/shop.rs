use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post, put},
};

use crate::{
    domain::state::{AppState, destock},
    http::{
        dto::{
            GenericAPIResponse,
            shop::{CheckoutWrite, StockRead, UpdateBasketQuery, UpdateStockQuery},
        },
        errors::APIError,
    },
};

pub fn shop_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/shop/stock", get(list_stock))
        .route("/shop/stock", put(add_stock))
        .route("/shop/basket", post(update_basket))
        .route("/shop/checkout", post(checkout))
}

// todo: paginate listing endpoints.
pub async fn list_stock(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<StockRead>>, APIError> {
    let stock = state.get_stock_read()?;
    let mut stock_read = Vec::<StockRead>::new();

    for (id, qtt) in stock.iter() {
        stock_read.push(StockRead {
            id: *id,
            amount: *qtt,
        });
    }

    Ok(Json(stock_read))
}

pub async fn add_stock(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Vec<UpdateStockQuery>>,
) -> Result<Json<GenericAPIResponse>, APIError> {
    let mut stock = state.get_stock_write()?;
    let mut prov_stock = state.get_prov_stock_write()?;

    for product in body {
        *stock.entry(product.id).or_insert(0) += product.amount;
        *prov_stock.entry(product.id).or_insert(0) += product.amount;
    }

    Ok(Json(GenericAPIResponse {
        result: "ok".to_string(),
    }))
}

pub(crate) async fn update_basket(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpdateBasketQuery>,
) -> Result<Json<GenericAPIResponse>, APIError> {
    let mut prov_stock = state.get_prov_stock_write()?;
    let mut baskets = state.get_baskets_write()?;

    // get basket if existes, or create new
    let basket = baskets.entry(body.id).or_insert(HashMap::new());
    let old_basket = basket.clone();
    let mut new_basket = HashMap::new();

    for update in body.basket {
        // if product id exists, and stock has enough (non reserved) overwrite basket quantity
        if let Some(stock_amount) = prov_stock.get(&update.id) {
            // "give back" stock reserved by this basket to stock
            let old_amount = old_basket.get(&update.id).unwrap_or(&0);

            if *stock_amount + *old_amount >= update.amount {
                new_basket.insert(update.id, update.amount);
            } else {
                return Err(APIError::JSONMessage(
                    StatusCode::BAD_REQUEST,
                    "oos".to_string(),
                ));
            }
        } else {
            return Err(APIError::JSONMessage(
                StatusCode::BAD_REQUEST,
                format!("product '{}' does not exist", update.id),
            ));
        }
    }

    // give back previous basket to prov stock
    for (id, qtt) in old_basket {
        *prov_stock.entry(id).or_insert(0) += qtt;
    }

    // update provisional stock and basket
    destock(&mut prov_stock, &new_basket)?;
    *basket = new_basket;

    Ok(Json(GenericAPIResponse {
        result: "ok".to_string(),
    }))
}

pub(crate) async fn checkout(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CheckoutWrite>,
) -> Result<Json<Vec<StockRead>>, APIError> {
    let mut stock = state.get_stock_write()?;
    let mut baskets = state.get_baskets_write()?;

    // remove basket content from stock
    let basket = baskets
        .get(&body.id)
        .ok_or(APIError::BadRequestMsg("basket does not exist".to_string()))?;

    destock(&mut stock, basket)?;

    // remove checked out basket
    let b = baskets.get(&body.id).cloned().unwrap();
    baskets.remove(&body.id);
    let mut ret = Vec::new();
    b.iter().for_each(|elt| {
        ret.push(StockRead {
            id: *elt.0,
            amount: *elt.1,
        })
    });

    Ok(Json(ret))
}
