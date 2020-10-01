fn main() {
    println!("Hello, world!");

    let y = {
        let a = 3;
        a + 2
    };

    println!("{}", y);

    another_function(10);

    //function return value
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

//function which return value -> 6
fn plus_one(x: i32) -> i32 {
    x + 1
}

//we define function using fn keyword and naming convention is snake case
fn another_function(x: i32) {
    println!("{}", x)
}
