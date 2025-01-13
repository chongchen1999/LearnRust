#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn generic_struct_demo() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("both_integer: {:?}", both_integer);
    println!("both_float: {:?}", both_float);
    println!("integer_and_float: {:?}", integer_and_float);
}