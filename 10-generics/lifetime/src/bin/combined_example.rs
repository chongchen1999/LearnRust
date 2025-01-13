fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let s3 = String::from("Today is someone's birthday!");

    let result = longest(s1.as_str(), s2, s3);
    println!("The longest string is {result}");
}

fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where 
    T: std::fmt::Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
