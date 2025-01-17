use std::rc::Rc;

fn main() {
    let rc = Rc::new(42); // Strong count = 1
    let weak1 = Rc::downgrade(&rc); // Weak count = 1
    let weak2 = Rc::downgrade(&rc); // Weak count = 2

    println!("Strong count: {}", Rc::strong_count(&rc)); // Output: 1
    println!("Weak count: {}", Rc::weak_count(&rc)); // Output: 2

    if let Some(strong) = weak1.upgrade() {
        println!("Upgraded value: {}", *strong); // Output: 42
    }

    drop(rc); // Strong count = 0, value is dropped
    println!(
        "Strong count: 0, Weak count: {}",
        Rc::weak_count(&Rc::new(0))
    );

    if weak2.upgrade().is_none() {
        println!("Value has been dropped"); // Output: Value has been dropped
    }
}
