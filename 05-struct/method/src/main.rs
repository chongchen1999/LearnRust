#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect.area());
    println!("{}", std::mem::size_of::<Rectangle>());
    println!("{}", std::mem::size_of_val(&rect));
    println!("{:#?}", rect);
}
