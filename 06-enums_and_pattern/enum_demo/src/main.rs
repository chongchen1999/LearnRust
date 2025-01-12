// Define an enum with associated data
enum IpAddr {
    V4(u8, u8, u8, u8), // IPv4 address
    V6(String),         // IPv6 address
}

// Implement a method for IpAddr
impl IpAddr {
    fn display(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(address) => println!("IPv6 Address: {}", address),
        }
    }
}

// Define an enum with multiple variant types
#[repr(u32)]
enum Message {
    Quit = 10,
    Move { x: i32, y: i32 } = 20,
    Write(String) = 33,
    ChangeColor(i32, i32, i32) = 233666,
}

// Implement a method for Message
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message received."),
            Message::Move { x, y } => println!("Move to position: ({}, {})", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // Using the IpAddr enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Display the IP addresses
    home.display();
    loopback.display();

    // Using the Message enum
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello, world!"));
    let change_color = Message::ChangeColor(255, 0, 0);

    // Call the methods on the Message enum
    quit.call();
    move_msg.call();
    write_msg.call();
    change_color.call();

    // Using the Option enum from the standard library
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // Handling the Option values
    match some_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("No number available."),
    }

    match no_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("No number available."),
    }
}
