use rust_complex_project::models::{Product, User};
use rust_complex_project::services::cart::Cart;

#[test]
fn test_cart() {
    let user = User::new("Bob".to_string(), "bob@example.com".to_string());
    let product = Product::new("Phone".to_string(), 499.99);

    let mut cart = Cart::new(user);
    cart.add_product(product);

    assert_eq!(cart.products.len(), 1);
}