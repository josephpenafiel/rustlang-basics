#![allow(dead_code)]
/*
MATCH ARMS
match arms consist of a pattern and an expression to run in the value
matches the arm's pattern:

match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION
}

match expressions need to be exhaustive.
The _ pattern will match anything, but it never binds to a variable.

IF LET EXPRESSIONS
if let expressions are used as a shorter wat to write the equivalent of a
match than only matches one case.

WHILE LET CONDITIONAL LOOPS
while let loop allows a while loop to run for as long as a pattern continues
to match.
*/

enum PATTERN {
    ONE,
    TWO,
    THREE,
}
fn main() {
    println!("Hello, world!");
    let a = PATTERN::ONE;
    let favorite: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "28".parse();

    match a {
        PATTERN::ONE => println!("pattern is one"),
        _ => println!("no"),
    }

    if let Some(color) = favorite {
        println!("yes {}", color);
    } else if is_tuesday {
        println!("yes it's tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("greater than 30");
        } else {
            println!("underage");
        }
    } else {
        println!("no favorite");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // .pop returns an option with the last element of a vec
        println!("{}", top);
    }

    // FOR LOOPS
    /*
    the enumerate method adapts an interator to produce a value and the
    index in the iterator, inside a tuple.
    */
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // MATCHING NAMED VARIABLES AND LITERALS

    let x = 1;

    match x {
        // this match statement could literally go to infinity
        1 => println!("yes"),
        2 => println!("yes"),
        _ => println!("no"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50!"),
        Some(num) => println!("Matched, some num x = {:?}", num),
        _ => println!("default case"),
    }

    //MULTIPLE PATTERNS AND RANGES

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    } // one or two

    let x = 5;

    match x {
        1...5 => println!("it's a number from 1 to 5"),
        _ => println!("some other number"),
    }

    let x = 'c';

    match x {
        'a'...'j' => println!("between a and j"),
        'k'...'z' => println!("between k and z",),
        _ => println!("something else"),
    }

    //DESTRUCTURING TO BREAK APART VALUES

    // DESTRUCTURING STRUCTS

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    /*
    above the vars a and b were created to match the pattern of the p variable
    that is of type Point.
    It can also be done as below
    */

    let Point { x, y } = p;
    println!("{}, {}", x, y); // 0, 7

    match p {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on neither axis: {}, {}", x, y),
    }

    // DESTRUCTURING ENUMS
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    let mut a = 0;

    match msg {
        Message::Quit => {
            println!("No data on Message::Quit");
        }
        Message::Move { x, y } => {
            println!("Move in x: {}, and y: {}", x, y);
        }
        Message::Write(text) => println!("Text Message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("change color r{}, g{}, b{}", r, g, b);
            a = g;
        }
    }

    println!("{}", a);

    //NESTED ENUM
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Display {
        ChangeColor(Color),
        On,
        Off,
    }

    let var = Display::ChangeColor(Color::Hsv(0, 160, 255));

    match var {
        Display::ChangeColor(Color::Rgb(r, g, b)) => println!("change red, green, blue"),
        Display::ChangeColor(Color::Hsv(h, s, v)) => println!("change hue, saturation, value"),
        _ => (),
    }

    /*
    we can use _ inside another pattern to ignore just part of a
    value.
    */

    let mut setting_value = Some(5);
    let new_setting_value = None;

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting {:?}", setting_value);

    //MATCH GUARDS

    let num = Some(6);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
} // end main
