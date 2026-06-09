use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStockQuery {
    pub id: u32,
    pub amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBasketQuery {
    pub id: u32,
    pub basket: Vec<UpdateStockQuery>,
}

#[derive(Debug, Serialize)]
pub struct StockRead {
    pub id: u32,
    pub amount: u32,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutWrite {
    pub id: u32,
}
