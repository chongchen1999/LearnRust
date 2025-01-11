fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{} {} {}", x, y, z);

    let index = [12, 13, 14, 15];
    let a: [i32; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // Ensure that the index is within bounds
    if index[2] < a.len() {
        println!("{}", a[index[2]]);
    } else {
        println!("Index out of bounds");
    }

    another_function(5, 6);

    let x = five(233);
    println!("{}", x);
}

fn another_function(x: i32, y: i32) {
    println!("{} {}", x, y);
}

fn five(x: i32) -> i32 {
    x + 5
}
