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

    //Tuple Type:- we can add multiple values of different types in tuple, and it is of fixed size, that means we can't add or remove values in tuple
    let tup = (500, 5.5, 'G');

    //Destructuring through pattern matching, to access the value of tuple
    let (d, e, f) = tup;

    println!("d: {}, e: {}, f: {} ", d, e, f);

    //we can also access the tuple elements using '.' followed by index of value
    let five_hundred = tup.0;

    println!("{}", five_hundred);

    //Array Type :- Unlike tuple, we have to store values of same type, array stores value of fixed length i.e  we can't add or remove elements, array is useful when we want to allocate data in stack
    let arr = [1, 2, 3, 4, 5];

    // we can access array elements with indexing
    let first = arr[0];

    print!("firstElement : {}", first);

    //alternate ways to create an array of same values
    let array2 = [3; 5]; // or let array2 = [3,3,3,3,3]

    println!("array with same value: {}", array2[0])
}
