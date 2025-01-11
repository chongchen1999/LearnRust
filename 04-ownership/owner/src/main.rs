use std::mem::size_of;

fn main() {
    let s = "123456789";
    println!("{}", s);

    // Size of the string slice type &str
    println!("{}", s.len()); 
    println!("{}", size_of::<&str>()); 

    let mut s = String::from("123456789");
    s.push_str(", 123456789");
    println!("{}", s);
    println!("{}", s.len());
    println!("{}", s.capacity());
    println!("{}", size_of::<String>());

    let s1 = String::from("123456789");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

    let s = String::from("123456789");
    take_ownership(s);

    let x = 5;
    make_copy(x);
    println!("{}", x);

    let s1 = give_ownership();

    let s2 = String::from("hello");
    println!("{}", s2);
    println!("Address of s2: {:p}", &s2);

    let s3 = take_and_give_back(s2);
    
    println!("{}", s1);
    println!("{}", s3);
    println!("Address of s3: {:p}", &s3);
}

fn give_ownership() -> String {
    let str = String::from("hello");
    str
}

fn take_and_give_back(str: String) -> String {
    str
}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn make_copy(i: u32) {
    println!("{}", i);
}
