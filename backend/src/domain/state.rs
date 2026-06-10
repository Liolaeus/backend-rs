use std::collections::hash_map::HashMap;

use axum::http::StatusCode;
use deadpool_diesel::postgres::Pool;

use crate::{config::AppConfig, controllers::errors::APIError};
pub type ProductID = u32;
pub type BasketID = u32;
pub type ProductAmount = u32;

// a stock is a simple list of product id / amout associations,
// baskets are a list of stocks associated to a basketID.
pub type Stock = HashMap<ProductID, ProductAmount>;

// since we store application data in the state, and may access it between handlers,
// we need it to be thread safe and referenced via copy of a pointer
#[derive(Clone)]
pub struct AppState {
    pub db: Pool,
    pub conf: AppConfig,
    // moap of uername: encrypted secret
    // pub users: RwLock<HashMap<String, Vec<u8>>>,
    // // map of productID: stock quantity
    // pub stock: RwLock<Stock>,
    // // represents the stock minus basketed items.
    // pub provisional_stock: RwLock<Stock>,
    // // map of productID: stock quantity
    // pub baskets: RwLock<HashMap<BasketID, Stock>>,
}

impl AppState {
    // pub fn get_users_map(
    //     &self,
    // ) -> Result<RwLockWriteGuard<'_, HashMap<String, Vec<u8>>>, APIError> {
    //     self.users
    //         .write()
    //         .map_err(|e| APIError::InternalLog(e.to_string()))
    // }
    // pub fn get_stock_read(
    //     &self,
    // ) -> Result<RwLockReadGuard<'_, HashMap<ProductID, ProductAmount>>, APIError> {
    //     self.stock
    //         .read()
    //         .map_err(|e| APIError::InternalLog(e.to_string()))
    // }
    // pub fn get_stock_write(
    //     &self,
    // ) -> Result<RwLockWriteGuard<'_, HashMap<ProductID, ProductAmount>>, APIError> {
    //     self.stock
    //         .write()
    //         .map_err(|e| APIError::InternalLog(e.to_string()))
    // }
    // pub fn get_prov_stock_write(
    //     &self,
    // ) -> Result<RwLockWriteGuard<'_, HashMap<ProductID, ProductAmount>>, APIError> {
    //     self.provisional_stock
    //         .write()
    //         .map_err(|e| APIError::InternalLog(e.to_string()))
    // }
    // pub fn get_baskets_write(
    //     &self,
    // ) -> Result<RwLockWriteGuard<'_, HashMap<BasketID, Stock>>, APIError> {
    //     self.baskets
    //         .write()
    //         .map_err(|e| APIError::InternalLog(e.to_string()))
    // }
}

// remove the content of a basket from a stock.
pub fn destock(stock: &mut Stock, basket: &Stock) -> Result<(), APIError> {
    for (id, qtt) in basket.iter() {
        if let Some(stock_qtt) = stock.get(id) {
            if *stock_qtt < *qtt {
                // should never happen
                return Err(APIError::JSONMessage(
                    StatusCode::BAD_REQUEST,
                    "oos".to_string(),
                ));
            }
            stock.insert(*id, *stock_qtt - qtt);
        } else {
            // should also never happed
            return Err(APIError::BadRequestMsg(format!(
                "product {} does not exist",
                id
            )));
        }
    }

    Ok(())
}
