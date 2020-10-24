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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 10,
    };
    println!("The area of rectangle is {}", rect.area());

    method_with_multiple_parameters();

    // associated function
    function_inside_impl_block();
}

fn method_with_multiple_parameters(){
    let rect1 = Rectangle {
        width: 50,
        height: 60
    };

    let rect2 = Rectangle {
        width: 40,
        height: 30
    };
    let rect3 = Rectangle {
        width: 60,
        height: 40
    };

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));
}

// associated function
fn function_inside_impl_block(){
 
   impl Rectangle{
       fn square(size: u32) -> Rectangle {
           Rectangle {
               width: size,
               height: size
           }
       }
   }

//    calling associated function, square
   let sq = Rectangle::square(10);

   println!("width: {} and height: {}", sq.width, sq.height);
}