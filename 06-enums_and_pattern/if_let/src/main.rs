enum Message {
    Text(String),
    Number(i32),
}

fn main() {
    // Example 1: Using `if let` with an enum
    let msg = Message::Text("Hello, Rust!".to_string());

    if let Message::Text(text) = msg {
        println!("Received text message: {}", text);
    } else {
        println!("Message is not a text variant");
    }

    // Example 2: Using `if let` with an Option
    let maybe_number = Some(42);

    if let Some(num) = maybe_number {
        println!("Found a number: {}", num);
    } else {
        println!("No number found");
    }

    // Example 3: Combining `if let` with a condition
    let another_number = Some(100);
    if let Some(num) = another_number {
        if num > 50 {
            println!("The number {} is greater than 50", num);
        }
    } else {
        println!("No number greater than 50 found");
    }

    // Example 4: Comparison to a match statement
    let msg2 = Message::Number(7);

    // Using `if let`
    if let Message::Number(value) = msg2 {
        println!("Number from if let: {}", value);
    }

    // Equivalent `match` statement
    match msg2 {
        Message::Number(value) => println!("Number from match: {}", value),
        _ => println!("Not a number"),
    }
}
