/*
Defining an Enum
Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs 
in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four 
and version six. These are the only possibilities for an IP address that our program will come across: we can enumerate all 
possible values, which is where enumeration gets its name.
*/

#[derive(Debug)]
enum IpAddrKind { // this is now a custom data type
    V4,
    V6
}

// define a struct that has an enum type 

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// we can put data directly into an enum
#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V4_2(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit, // has no data associated at all
    Move {x: i32, y:i32}, // has an anonymous struct
    Write(String), // takes a String
    ChangeColor(i32, i32, i32) // includes three i32 values
}

// we can implement methods on enums 

impl Message {
    fn call(&self) {
        //function implementation
    }
}

/* 
    let m = Message::Write(String::from("hello"));
    m.call();
*/

fn main() {

    println!("Hello, world!");
    // creating instances of IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // defining a struct with an enum type

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //defining an enum with data inside enum variant

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let another_home = IpAddrEnum::V4_2(127,0,0,1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    println!("{:#?}", home);


   
}


// we can define funtions that take an type of enum
fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}
