pub fn push_str_demo() {
    let mut s = String::from("foo");
    s.push_str("______bar");
    println!("{}", s);
}

pub fn concat_string_demo() {
    let s1 = String::from("Hello, ");
    println!("address of s1: {:p}", &s1);

    let s2 = String::from("world!");
    println!("address of s2: {:p}", &s2);

    // Note s1 has been moved here and can no longer be used
    let s3 = s1 + &s2; 
    println!("{}", s3);
    println!("address of s3: {:p}", &s3);
}

pub fn format_demo() {
    let s1 = String::from("tic");
    let s2 = "tac";
    let s3 = format!("{}-{}-{}", s1, s2, "toe");

    println!("{}", s3);
}