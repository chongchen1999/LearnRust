// Define a generic Point struct with one generic type T
struct Point<T> {
    x: T,
    y: T,
}

// Implement a generic method `x` for Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement a method specific to Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Define a generic Point struct with two generic types X1 and Y1
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// Implement a generic method `mixup` for Point2<X1, Y1>
impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Example 1: Basic generic method
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x()); // Output: p.x = 5

    // Example 2: Method specific to a concrete type
    let p_float = Point { x: 3.0, y: 4.0 };
    // Output: Distance from origin: 5
    println!("Distance from origin: {}", p_float.distance_from_origin());

    // Example 3: Mixing different generic types in methods
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // Output: p3.x = 5, p3.y = c
}
