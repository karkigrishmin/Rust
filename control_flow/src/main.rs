fn main() {
    //-----if expression-------
    let x = 10;
    if x > 15 {
        println!("X is greater");
    }
    println!("X is smaller");

    let number = 5;

    if number != 0 {
        println!("Number is not zero");
    }

    //we  have to always  provide boolean value in the if condition, the following code will bring error
    // let num = 5;

    // if num {
    //     println!("true condition")
    // }

    //using if in a let statement
    using_if();
}

//using if in a let statement
fn using_if() {
    let condition = true;

    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);

    //below code,using return value of different type will lead us to the error because in Rust , the variable should be of single type
    // let condition = true;

    // let num = if condition { 5 } else { "six" };
    // println!("The value of num is: {}", num);
}
