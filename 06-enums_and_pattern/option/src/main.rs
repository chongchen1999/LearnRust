fn main() {
    // Example 1: Defining Option<T> values
    let some_number: Option<i32> = Some(10); // A value of 10
    let absent_number: Option<i32> = None; // No value (None)

    // Example 2: Using `match` to handle Option<T>
    match some_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("No number found!"),
    }

    match absent_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("No number found!"),
    }

    // Example 3: Handling Option<T> with `unwrap_or`
    // This provides a default value if the Option is None.
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    let x_value = x.unwrap_or(0); // If x is None, use 0 as the default
    let y_value = y.unwrap_or(0); // If y is None, use 0 as the default
    println!("x_value: {}, y_value: {}", x_value, y_value);

    // Example 4: Combining Option<T> with arithmetic
    // Use `map` to safely transform the value inside an Option.
    let number = Some(10);
    let double_number = number.map(|n| n * 2); // Multiply the value by 2 if it's Some
    println!("Double the number: {:?}", double_number);

    // Example 5: Using Option<T> with `unwrap` (be cautious!)
    // `unwrap` will panic if the value is None.
    let some_value = Some(42);
    // let none_value: Option<i32> = None;

    println!("Value in some_value: {}", some_value.unwrap());
    // The line below will cause a panic if uncommented:
    // println!("Value in none_value: {}", none_value.unwrap());

    // Example 6: Chaining methods
    // Use `and_then` to handle nested Options.
    let nested_option = Some(Some(7));
    let flattened_option = nested_option.and_then(|inner| inner);
    println!("Flattened Option: {:?}", flattened_option);
}
