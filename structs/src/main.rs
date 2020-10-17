fn main() {
    //defining  User struct, here we define name and type of the data which is also called fields
    struct User {
        name: String,
        email: String,
        active: bool,
    }

    //creating an instance of User
    let mut user1 = User {
        name: String::from("Grishmin"),
        email: String::from("grishmink@gmail.com"),
        active: true,
    };

    user1.name = String::from("Grish");

    //we can access the value of field using dotnotation
    println!("{}", user1.name);
    //creating second instance of User, here we are asigning new value for name and also using user1 instance field values
    let user2 = User {
        name: String::from("Grish"),
        email: user1.email,
        active: user1.active,
    };
    println!("{}, {}", user2.email, user2.active);

    //we can also use struct update syntax to use the data from the field of user1 instance
    struct_update_syntax_example();

    // tuple struct
    tuple_struct();

    // let name = String::from("Grish");
    // user_instance(name);
}

// use of struct update syntax
fn struct_update_syntax_example() {
    struct Customer {
        name: String,
        age: i32,
    }

    // First Customer instance
    let customer1 = Customer {
        name: String::from("Ram"),
        age: 20,
    };
    println!("{}", customer1.name);

    // Second Customer instance and also using same field of first Customer instance, customer1, using struct update syntax
    let customer2 = Customer { ..customer1 };
    println!("{}, {}", customer2.name, customer2.age);
}

// Tuple struct
fn tuple_struct() {
    struct Point(i32, i32, i32);
    let origin = Point(1, 2, 3);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);
}

// This code is just for the concept of shorthand, it will not run
// fn user_instance(name:String) -> User {
//     User {
//         // if key and value are identical then we can just use key, so here we can just use name variable as it is passed as a function paramter
//         // name: name,
//         name,
//         count: 0,
//     }
// }
