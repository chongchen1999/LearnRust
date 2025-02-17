#[derive(PartialEq, Debug, Clone)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    println!("Before: {:?}", shoes);

    let shoes2: Vec<Shoe> = shoes.iter().filter(|s| s.size == 10).cloned().collect();

    println!("{:?}", shoes);
    println!("{:?}", shoes2);

    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:?}", in_my_size);
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
