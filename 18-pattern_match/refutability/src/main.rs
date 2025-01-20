fn main() {
    // Irrefutable pattern: `let` statement
    let x = 5; // `x` will always match, so this is irrefutable
    println!("x = {}", x);

    // Refutable pattern: `if let` with `Option`
    let some_option_value: Option<i32> = Some(10);

    if let Some(y) = some_option_value {
        // `Some(y)` is refutable
        println!("y = {}", y);
    } else {
        println!("The value is None");
    }

    // Refutable pattern: `match` with `Option`
    let another_option_value: Option<i32> = None;

    match another_option_value {
        Some(z) => println!("z = {}", z),      // `Some(z)` is refutable
        None => println!("The value is None"), // `None` is refutable
    }

    // Irrefutable pattern in `match` (catch-all arm)
    let value = 42;

    match value {
        42 => println!("The value is 42"),            // Refutable pattern
        _ => println!("The value is something else"), // `_` is irrefutable
    }

    // Error: Using a refutable pattern in a `let` statement (uncomment to see the error)
    // let Some(w) = some_option_value; // This will cause a compile-time error

    // Fix: Use `if let` for refutable patterns
    if let Some(w) = some_option_value {
        println!("w = {}", w);
    } else {
        println!("The value is None");
    }

    // Warning: Using an irrefutable pattern in `if let` (uncomment to see the warning)
    // if let v = 5 { // `v` is irrefutable
    //     println!("v = {}", v);
    // }

    // Fix: Replace `if let` with `let` for irrefutable patterns
    let v = 5;
    println!("v = {}", v);
}
