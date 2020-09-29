fn main() {
    //mutable variable
    // let mut x = 5;

    // println!("value of x is: {}", x);

    // x = 6;

    // println!("value of x is: {}", x)

    //Shadowing
    let x = 5;

    let x = x + 1;

    let x = x * 3;

    println!("Value of x is: {}", x)
}
