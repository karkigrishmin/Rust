fn main() {
    //string literal, we cant mutate or add literals, so it is stored in stack.
    let s = "Hello";
    println!("{}", s);

    //String Type created using from function, we can also mutate String type, this type allocates a heap memory where memory size is grown if needed.
    let mut str = String::from("Hello");
    str.push_str(", world");
    println!("{}", str);

    //assigning one String value to another variable
    // when we assign our String variable s1 to s2, group of data stored in stack of s1 gets copied to s2 and now both s1 and s2's data pointer points to same heap memory, so as both variables are pointing to same memory, it will also try to free the same memmory twice when the variable goes out of scope , and the error occurs, So Rust consider s1 variable as it is not valid , now memory is freed only one time, so we can only use s2 variable
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    //using clone method-> copies data of heap memory and gets assigned in a2, so here both a1 and a2 have separate heap memory, so a1 is also considered as valid.
    let a1 = String::from("Grishmin");
    let a2 = a1.clone();
    println!("a1= {} and a2= {}", a1, a2);

    ownership_function();
}

fn ownership_function() {
    let s = String::from("Hello grish"); //s comes into scope
    takes_ownership(s); //s is moved to the function
                        //so here s is not valid

    let i = 5; //i comes into scope

    makes_copy(i); //value of i is copied because integer type is Copy
                   //i still valid
}

fn takes_ownership(str_value: String) {
    println!("{}", str_value);
} //str_value goes out of scope and memory is freed

fn makes_copy(integer_value: i32) {
    println!("{}", integer_value);
} //integer_value goes out of scope and nothing happens
