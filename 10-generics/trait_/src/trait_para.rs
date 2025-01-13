use std::fmt::Display;

// Define the Summary trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implement Summary for NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.content)
    }
}

// Implement Summary for Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Using `impl Trait` for a single parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Using trait bound syntax
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple parameters with `impl Trait`
pub fn notify_multiple(item1: &impl Summary, item2: &impl Summary) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// Enforcing the same type for multiple parameters
pub fn notify_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

pub fn notify_multiple_traits<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display: {}", item);
}

// Main function to demonstrate usage
pub fn trait_as_parameter_demo() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // Using `impl Trait`
    notify(&article);
    notify(&tweet);

    // Using trait bound syntax
    notify_trait_bound(&article);
    notify_trait_bound(&tweet);

    // Multiple parameters with `impl Trait`
    notify_multiple(&article, &tweet);

    // Enforcing the same type for multiple parameters
    notify_same_type(&article, &article); // Both must be NewsArticle

    // Specifying multiple trait bounds
    notify_multiple_traits(&article); // NewsArticle implements both Summary and Display
}
