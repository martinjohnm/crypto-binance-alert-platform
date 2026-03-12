use std::{cell::RefCell, collections::HashMap};


#[derive(Debug)]
pub struct MarketState {
    pub price : f64
}

#[derive(Debug)]
pub struct MarketManager {
    markets : RefCell<HashMap<String, MarketState>>
}

impl MarketManager {
    pub fn new() -> Self {
        Self { markets: RefCell::new(HashMap::with_capacity(1000)) }
    }

    pub fn update_price(&self, symbol : &str, price : f64) {
        self.markets.borrow_mut().insert(symbol.to_string(), MarketState { price });
    }
}