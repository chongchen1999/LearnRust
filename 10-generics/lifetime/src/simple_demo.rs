fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn example1() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn example2() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}

fn example3() {
    fn longest_return_first<'a>(x: &'a str, y: &str) -> &'a str {
        println!("{:?}", y.as_ptr());
        x
    }

    let string1 = String::from("long string is long");
    let result2 = longest_return_first(string1.as_str(), "short");
    println!("The first string is {}", result2);
    let x = "short";
    println!("{:?}", x.as_ptr());
}

fn example4() {
    fn longest_invalid<'a>(_x: &'a str, _y: & str) -> String {
        let result = String::from("really long string");
        result
    }
    let string1 = String::from("long string is long");
    let result = longest_invalid(string1.as_str(), "short");
    println!("The longest string is {}", result);
}

pub fn run() {
    example1();
    example2();
    example3();
    example4();
}

pub fn lifetime_elision() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }
    
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("The first word of my_string is: {}", word);


    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal);
    println!("The first word of my_string_literal is: {}", word);

    let word = first_word(my_string_literal);
    println!("The first word of my_string_literal is: {}", word);
}