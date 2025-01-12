use std::any::type_name;

// Helper function to get the type of a variable
fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub fn to_string_demo() {
    let data = "Hello, world!";
    let s = data.to_string();
    let s1 = "Initial contents".to_string();
    let s2 = String::from("Initial contents");

    println!("datatype of data: {}", type_of(&data));
    println!("datatype of s1: {}", type_of(&s1));
    println!("datatype of s: {}", type_of(&s));
    println!("datatype of s2: {}", type_of(&s2));
}