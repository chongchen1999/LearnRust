#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 300,
        height: 500,
    };
    println!("{}", rect.area());
    println!("{}", std::mem::size_of::<Rectangle>());
    println!("{}", std::mem::size_of_val(&rect));
    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    let s = Rectangle::square(20);
    println!("{:#?}", s);
}
