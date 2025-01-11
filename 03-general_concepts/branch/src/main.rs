fn main() {
    let number = 3;
    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is not less than 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    println!("{}", loop_example());
    for_example();
}

fn loop_example() -> i32 {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    result
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for x in a.iter() {
        println!("{}!", x);
    }
}
