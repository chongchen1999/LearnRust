struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn user_example() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("roronoa@example.com");
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{}", user1.active);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    user_example();
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", area(&rect));
    println!("{}", std::mem::size_of::<Rectangle>());
    println!("{}", std::mem::size_of_val(&rect));
    println!("{:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
