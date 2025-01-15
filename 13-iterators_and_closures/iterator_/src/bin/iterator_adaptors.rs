fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    println!("v1: {:?}", v1);

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}
