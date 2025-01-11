fn main() {
    println!("{}", std::mem::size_of::<String>());
    println!("{}", std::mem::size_of::<&String>());

    let mut s1 = String::from("Hello");
    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}
