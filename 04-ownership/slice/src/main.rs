fn main() {
    let s = String::from("hello world ---------------------");
    let word_index = first_word(&s);

    println!("{}", word_index);

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    println!("size of String is: {}", std::mem::size_of::<String>());
    println!("size of &str is: {}", std::mem::size_of::<&str>());
    println!("{}", std::mem::size_of_val(&s));
    println!("{}", std::mem::size_of_val(&hello));

    let my_string = String::from("hello world");
    // println!("size of &String is: {}", std::mem::size_of_val(&my_string));
    let word_index = first_word(&my_string);
    println!("{}", word_index);

    let my_string_literal = "hello world";
    let word_index = first_word(my_string_literal);
    println!("{}", my_string_literal);
    println!("{}", word_index);
}

fn first_word(s: &str) -> &str {
    println!("size of datatype of s is: {}", std::mem::size_of_val(&s));
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
