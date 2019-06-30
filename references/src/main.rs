fn main() {
    println!("Hello, world!");

    // === REFERENCES and BORROWING === //
    let s1 = String::from("hello");
    let len = calculate_len(&s1); // passing a reference of s1
    println!("length of {} is {}", s1, len);
    println!("s1 still usable!: {}", s1);

    /*
    & is called a reference, they allow to refer to some value without taking ownership
    the &s1 syntax lets us create a reference that refers to the value of s1. Because it 
    doesn't own s1, the value it points to will not be dropped when the reference goes 
    out of scope
    */

    //let s = String::from("hello");
    // change_borrowed(&s); // can't do because references are immutable by default

    //the following code will work
    let mut s = String::from("hello");
    println!("mutable String {}", s);
    can_change_borrowed(&mut s);
    println!("s1 changed to: {}", s);

    /*
    mutable references have one big restriction: you can have only one mutable reference 
    to a particular piece of data in a particular scope. This code will fail:
    
    let mut s = String::from("hello");
    let r1 = &mut s; // this is enough
    let r2 = &mut s; // too much

    this prevents data race

    the following code will work:
    */

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1: {}", r1)
    } // scope ends

    let r2 = &mut s;
    println!("r2: {}", r2);

    /*
    A similar rule exists for combining mutable and immutable references. This code results in an error:

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    we can't have immutable and mutable references of the same data at the same time

    the following code will work:
    */

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, and r2: {}", r1,r2);
    // r1 and r2 we already used
    let r3 = &mut s;
    println!("r3 before mut: {}", r3);
    r3.push_str(", world!");
    println!("r3 after mut: {}", r3);
    // println!("r1: {}, and r2: {}", r1,r2); //cant use these anymore
    println!("original s: {}",s);

    // === DANGLING REFERENCES === //

    // let reference_to_nothing = dangle();

    /*
    code above doesn't compile because when the function goes out of scope
    the reference to the local var is dropped, so we're left if a dangling 
    refence. See dangle() function's definition
    */


}

// fn dangle() -> &String { // returns refence to a String
//     let s = String::from("hello"); // s is the new string
//     &s; // return a reference
// } // s goes out of scope and it's memory goes away

fn calculate_len(s: &String) -> usize {
    s.len()
}

// fn change_borrowed(some_string: &String) {
//     some_string.push_str(", world");
// }

fn can_change_borrowed(some_string: &mut String) {
    some_string.push_str(", world!");
}
