fn main() {
    let mut x = 5;
    println!("x: {}", x);
    println!("address of x is: {:p}", &x);

    {
        let y = &x; // immutable reference
        println!("y: {}", y);
        println!("address of y is: {:p}", &y);
    }

    {
        let z = &mut x; // mutable reference
        *z += 1;
        println!("z: {}", z);
        println!("address of z is: {:p}", &z);
    } // z goes out of scope here

    // Now we can access x again
    println!("x: {}", x);
    println!("address of x is: {:p}", &x);

    let a: i32 = 42;
    let b: i64 = 42;
    let c: u8 = 42;
    let s = String::from("Hello");

    let ref_a = &a;
    let ref_b = &b;
    let ref_c = &c;
    let ref_s = &s;

    println!(
        "Size of &i32: {} bytes, value: {}",
        std::mem::size_of::<&i32>(),
        ref_a
    );
    println!(
        "Size of &i64: {} bytes, value: {}",
        std::mem::size_of::<&i64>(),
        ref_b
    );
    println!(
        "Size of &u8: {} bytes, value: {}",
        std::mem::size_of::<&u8>(),
        ref_c
    );
    println!(
        "Size of &String: {} bytes, value: {}",
        std::mem::size_of::<&String>(),
        ref_s
    );
}
