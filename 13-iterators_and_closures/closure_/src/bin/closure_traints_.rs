fn main() {
    // Example 1: Fn closure
    // Fn closures can be called multiple times and do not mutate the captured environment.
    let x = 5;
    let square = |num: i32| -> i32 { num * x };
    println!("Square of 4: {}", square(4)); // Output: Square of 4: 20
    println!("Square of 6: {}", square(6)); // Output: Square of 6: 30

    // Example 2: FnMut closure
    // FnMut closures can be called multiple times and can mutate the captured environment.
    let mut y = 10;
    let mut increment = |num: i32| {
        y += num;
        y
    };
    println!("Incremented y by 3: {}", increment(3)); // Output: Incremented y by 3: 13
    println!("Incremented y by 5: {}", increment(5)); // Output: Incremented y by 5: 18

    // Example 3: FnOnce closure
    // FnOnce closures can only be called once because they consume the captured environment.
    let z = String::from("Hello");
    println!("address of z: {:p}", &z);
    println!("address of z points to: {:p}", z.as_ptr());

    let consume = |s: String| -> String {
        println!("Consumed string: {}", s);
        s
    };
    let result = consume(z); // z is moved here
    println!("Result: {}", result); // Output: Consumed string: Hello, Result: Hello
    println!("address of result: {:p}", &result);
    println!("address of result points to: {:p}", result.as_ptr());

    // Uncommenting the following line would cause a compile-time error because z has been moved.
    // println!("Trying to use z again: {}", z);
}
