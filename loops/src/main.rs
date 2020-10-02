fn main() {
    //loops:- loop, while and for

    //loop -> code inside the loop block runs continously until we explicitly stop the program
    // loop {
    //     println!("again");
    // }

    //we can use break keyword to stop executing loop again and again when we want
    loop {
        println!("again");
        break;
    }

    //returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    //while loop
    use_of_while();

    //for loop
    use_of_for();

    //countdown example
    countdown();
}

//while loop
fn use_of_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number = number - 1;
    }

    println!("out of loop");
}

//looping through array using for loop
fn use_of_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The vaue is: {}", element);
    }
    println!("out of loop");
}

//countdown example using for loop
fn countdown() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("out of loop");
}
