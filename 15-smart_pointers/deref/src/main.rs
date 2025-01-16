use std::ops::Deref;

// Define a custom smart pointer `MyBox<T>`
struct MyBox<T>(T);

// Implement a constructor for `MyBox<T>`
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement the `Deref` trait for `MyBox<T>`
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// A function that takes a string slice (`&str`) as an argument
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // Create a `MyBox<String>` instance
    let my_box = MyBox::new(String::from("Rustacean"));

    // Call the `greet` function with a reference to `MyBox<String>`
    // Deref coercion automatically converts `&MyBox<String>` to `&str`
    greet(&my_box);

    // Manually demonstrate the steps of deref coercion
    let string_reference: &String = my_box.deref(); // Step 1: `&MyBox<String>` -> `&String`
    let str_slice: &str = string_reference.as_str(); // Step 2: `&String` -> `&str`
    greet(str_slice); // Call `greet` with `&str`
}
