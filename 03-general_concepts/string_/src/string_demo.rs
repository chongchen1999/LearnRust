pub fn run() {
    let x = "xyz";
    println!("address of x points to: {:?}", x.as_ptr());

    let y = "xyz";
    println!("address of y points to: {:?}", y.as_ptr());

    let z = y;
    println!("address of z points to: {:?}", z.as_ptr());

    println!("{}", y);
}

pub fn string_slices() {
    let s = String::from("hello");
    let s1 = &s[..];
    let s2 = &s;
    println!("{}", s1);
    println!("{}", s2);
}

pub fn vector_slices() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let v1 = &v[..];
    let v2: &[i32] = &v[..];
    println!("{:?}", v1);
    println!("{:?}", v2);
}