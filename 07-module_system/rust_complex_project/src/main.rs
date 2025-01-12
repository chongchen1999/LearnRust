use rust_complex_project::models::{Product, User};
use rust_complex_project::services::{cart::Cart, inventory::Inventory};
use rust_complex_project::utils::logger::log_message;

fn main() {
    log_message("Starting e-commerce system...");

    let user = User::new("Alice".to_string(), "alice@example.com".to_string());
    let product = Product::new("Laptop".to_string(), 999.99);

    let mut inventory = Inventory::new();
    inventory.add_product(product.clone());

    let mut cart = Cart::new(user);
    cart.add_product(product);

    println!("{:?}", cart);
}
