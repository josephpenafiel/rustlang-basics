#![allow(unused_variables)]
fn main() {

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    println!("Hello, world!");

    let v: Vec<i32> = Vec::new(); // create new empty vector
    let v = vec![1,2,3,4]; // create a vector with values
    let mut v = Vec::new(); 

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8); // vector's memory is freed when they go out of scope 

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[3];

    match v.get(2) {
        Some(value) => println!("value is {}", value),
        None => println!("there's no third element"),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 200, 300];
    for value in &mut v { // mutable reference
        *value += 50;   // dereference operator *
        println!("{}", value);
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
