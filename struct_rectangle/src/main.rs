 
fn main() {
    let width = 10;
    let height = 20;

    println!(
        "The area of reactangle is {} square pixels.",
        area(width, height)
    );

    // Example using tuple
    using_tuple();

    // same calculation of rectangle using struct
    using_struct();

    print_struct_instance();
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

// Calculating area using tuple
fn using_tuple() {
    let rect = (20, 30);
    println!("The area of rectangle is {}", area_using_tuple(rect));
}

fn area_using_tuple(reactangle: (u32, u32)) -> u32 {
    reactangle.0 * reactangle.1
}

// struct definition
 #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn using_struct() {
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("The area of rectangle is {}", area_using_struct(&rect1));
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


// useful for debugging
fn print_struct_instance(){
  

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };
    
    println!("rect2 is {:#?}", rect2);
    println!("rect2 is {:?}", rect2);
}