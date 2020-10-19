fn main() {
    let width = 10;
    let height = 20;

    println!(
        "The area of reactangle is {} square pixels.",
        area(width, height)
    );
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}
