fn main() {
    //Datatypes
    //Scalar Types : Integer, floating point , Boolean, Character type
    //Integer
    let x = 5;

    //Floating point
    let y = 2.55;

    //Boolean
    let a = true;

    let b: bool = false;

    //Character Type
    let c = 'Z';

    println!("Value of integer x is x: {},value of floating point y is y: {}, boolean a and b is {}, {} and character c is : {}  ", x, y, a, b, c);

    //Compound Types :- Tuple and array

    //Tuple Type
    let tup = (500, 5.5, 'G');

    //Destructuring through pattern matching, to access the value of tuple
    let (d, e, f) = tup;

    println!("d: {}, e: {}, f: {} ", d, e, f);

    //we can also access the tuple elements using '.' followed by index of value
    let five_hundred = tup.0;

    println!("{}", five_hundred);
}
