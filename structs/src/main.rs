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

    // let name = String::from("Grish");
    // user_instance(name);
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
