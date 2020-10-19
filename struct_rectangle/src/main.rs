fn main() {
    let width = 10;
    let height = 20;

    println!(
        "The area of reactangle is {} square pixels.",
        area(width, height)
    );

    // Example using tuple
    using_tuple();
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
