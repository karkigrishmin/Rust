fn main() {
    //mutable variable
    let mut a = 5;

    println!("value of a is: {}", a);

    a = 6;

    println!("value of a is: {}", a);

    //Shadowing
    //Shadowing and mut are different, we can specify type of second variable in shadowing
    let x = 5;

    let x = x + 1;

    let x = x * 3;

    println!("Value of x is: {}", x);

    //----Constant-----, we have to specify type of the constant as well
    //underscore and uppercase is the naming convention
    const MAX_VALUE: u32 = 100_000;

    println!("max value is: {}", MAX_VALUE)
}
