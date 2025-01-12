use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)] // Add `Clone` here
pub struct Product {
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(name: String, price: f64) -> Self {
        Product { name, price }
    }
}