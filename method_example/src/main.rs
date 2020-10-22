//defining struct
struct Rectangle {
    width: u32,
    height: u32,
}

// defining method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 10,
    };

    println!("The area of rectangle is {}", rect.area());
}
