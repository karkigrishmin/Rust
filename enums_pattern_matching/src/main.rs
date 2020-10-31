fn main() {
    enum_example_one();
    enum_example_two();
    enum_example_three();
    enum_example_four();
    enum_example_five();
    
    // Option enum
    option_enum();
}

fn enum_example_one(){
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // instances of struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!(" {}", home.address);
}

// example two
fn enum_example_two(){
    // storing data directly into the variants of enum
    enum IpAddr{
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

// example three
fn enum_example_three(){
    // enum variants can store different types  and amount of data
    enum IpAddr {
        V4(u8, u8, u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));
}

// example four
fn enum_example_four(){
    //we can put any kind of data inside enum variants like String, numeric types, struct
    struct IpV4Addr{
        // code
    }

    struct IpV6Addr {
        // code
    }

    enum IpAddr {
        V4(IpV4Addr),
        V6(IpV6Addr),
    }
}

// example five
fn enum_example_five(){
    //enum variants that can store different types of data
    enum Message {
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }

    // structs can also hold data as above enum
    // struct QuitMessage;  //unit struct

    // struct MoveMessage {
    //     x:i32, 
    //     y:i32,
    // }

    // struct WriteMessage(String); //Tuple struct

    // struct ChangeColorMessage(i32,i32,i32); //tuple struct

    // we can also define methods in enum
    impl Message {
        fn call(&self) {
            // body
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();
}

// Option enum
fn option_enum(){
    enum Option<T> {
        Some(T),
        None
    }

    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;
  
    //to use the value from Option<T> we need to use match expression to handle each variant of Option enum.
}