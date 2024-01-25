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
    let _scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels.", rect1.area());
    // println!("The rectange is {:?}", rect1);
    // dbg!(&rect1);
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
