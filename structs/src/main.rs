// === Defining Structs === // 
#[derive(Debug)] // so that we can print a struct
struct Rectangle {
        width: u32,
        height: u32
    }

// adding a method to the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //can add more methods

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //can have associated functions (don't take self as parameter)
    fn square(size: u32) -> Rectangle { // returns a rectangle object 
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10
    };

    let sqr = Rectangle::square(5);

    println!("area of rect1 is: {}", area(&rect1));
    println!("area with struct's method: {}", rect1.area());
    // can also call a method like (&rect1).area(); but rust has automatic referencing

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("i'm a square {:#?}", sqr);

    println!("printing a struct: {:#?}", rect1);    
}

fn area(rect: &Rectangle) -> u32 { // passing a reference to a struct Rectangle
    rect.width * rect.height
}
