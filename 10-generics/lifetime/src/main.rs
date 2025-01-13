fn main() {
    lifetime::simple_demo::run();
    println!("{}", "-".repeat(50));
    lifetime::struct_ref::run();
    println!("{}", "-".repeat(50));
    lifetime::simple_demo::lifetime_elision();
}
