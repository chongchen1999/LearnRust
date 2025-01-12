use crate::models::{Product, User};

#[derive(Debug)]
pub struct Cart {
    pub user: User,
    pub products: Vec<Product>,
}

impl Cart {
    pub fn new(user: User) -> Self {
        Cart {
            user,
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
}
