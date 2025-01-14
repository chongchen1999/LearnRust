#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn same_size_rectangles() {
        let rect1 = Rectangle {
            width: 5,
            height: 5,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(rect1.can_hold(&rect2));
        assert!(rect2.can_hold(&rect1));
    }

    #[test]
    fn one_dimension_larger() {
        let rect1 = Rectangle {
            width: 8,
            height: 5,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 8,
        };

        assert!(!rect1.can_hold(&rect2));
        assert!(!rect2.can_hold(&rect1));
    }
}
