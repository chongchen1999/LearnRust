pub fn closure_fnonce() {
    let x = String::from("Hello");
    println!("address of String x is {:p}", &x);

    let consume_closure = || {
        println!("{}", x);
        println!("address of String x is {:p}", &x);
        // std::mem::drop(x); // Explicitly drop `x` to show it's consumed.
    };

    consume_closure(); // Works fine the first time.
                       // consume_closure();
}

pub fn closure_fnmut() {
    let mut x = 5;

    // This closure mutates `x` but does not consume it.
    let mut mutate_closure = || {
        x += 1;
        println!("x is now: {}", x);
    };

    mutate_closure(); // x is now: 6
    mutate_closure(); // x is now: 7
    mutate_closure(); // x is now: 8
}

pub fn closure_fn() {
    let x = 10;

    // This closure captures `x` immutably and does not modify it.
    let read_closure = || {
        println!("x is: {}", x);
    };

    read_closure(); // x is: 10
    read_closure(); // x is: 10
    read_closure(); // x is: 10
}
