fn main() {
    println!("Hello, world!");

    another_function(10);
}

//we define function using fn keyword and naming convention is snake case
fn another_function(x: i32) {
    println!("{}", x)
}
