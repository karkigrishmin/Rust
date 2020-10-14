fn main() {
    //defining  User struct, here we define name and type of the data which is also called fields
    struct User {
        name: String,
        email: String,
        active: bool,
    }

    //creating an instance of User
    let user1 = User {
        name: String::from("Grishmin"),
        email: String::from("grishmink@gmail.com"),
        active: true,
    };
    //we can access the value of field using dotnotation
    println!("{}", user1.name);
}
