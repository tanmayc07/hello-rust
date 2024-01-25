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

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let _scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(3);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square = {square1:?}");
    println!("area of square = {}", square1.area());

    // println!("The area of rectangle is {} square pixels.", rect1.area());
    // println!("The rectange is {:?}", rect1);
    // dbg!(&rect1);
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
