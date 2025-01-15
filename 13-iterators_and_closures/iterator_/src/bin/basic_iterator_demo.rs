fn main() {
    let v = vec!["11", "22", "33", "44", "55"];

    // Create an iterator over the vector
    let mut iter = v.iter();
    println!("{}", std::mem::size_of_val(&iter));
    println!("{:p}", &"11");
    println!("{:p}", &"22");
    println!("{:p}", &"33");
    println!("{:p}", &"44");
    println!("{:p}", &"55");

    loop {
        match iter.next() {
            Some(x) => println!("Value: {}, address: {:p}", x, x),
            None => break,
        }
    }

    println!("{}", std::mem::size_of_val(&iter));
}
