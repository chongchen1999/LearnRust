fn main() {
    println!("Welcome to My Rust Project!");

    // Call functions from the library crate
    demo_project::greet();

    // Call a function from the utils module
    let sum = demo_project::utils::math::add(5, 3);
    println!("Sum: {}", sum);
}
