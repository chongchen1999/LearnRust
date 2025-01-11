const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let mut x: u32 = x + 1;
    println!("The value of x is: {}", x);
    x = x + 10;
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINT is {}", MAX_POINTS);

    let y: u32 = "42".parse().expect("Not a number!");
    println!("The value of y is: {}", y);
}
