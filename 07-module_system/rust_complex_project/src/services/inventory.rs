use crate::models::Product;

#[derive(Debug)]
pub struct Inventory {
    pub products: Vec<Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
}