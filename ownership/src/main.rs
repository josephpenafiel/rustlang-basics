/*
First, let’s take a look at the ownership rules. Keep these rules in mind 
as we work through the examples that illustrate them:

Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    println!("Hello, world!");

    // variable scope 

    // let one_s = "hello"; // non mutable
    // let mut another_s = String::from("hello"); // mutable

    // let x = 5; 
    // let y = x; // totally ok

    let s1 = String::from("hello");
    let s2 = s1; // s1 no longer available

    println!("s2: {}", s2);

    /*
    declaring s1 creates 3 parts: 
                S1
        name     |   value               index     value
        ptr      |   address     ->        0         h
        len      |   5                     1         e
        capacity |   5                     2         l

        length is how much memory (Bytes) the content of s1 is using (hello = 5 bytes)
        capacity is the total amount of memory
        doing let s2 = s1 is actually copying the ptr from s1 to s2 resulting in having
        two vars pointing at the same address. What the compiler does is invalidates s1,
        so it's no longer usable. Now the var s2 is valid.

    */

    /*
    the code below will make another table like above but with a different address, pointing
    at a block which has the same values as s1
    */

    let s3 = s2.clone();

    println!("s3: {}", s3);

    // === OWNERSHIP and FUNCTIONS === //

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value move into the function...
                        // s is no longer valid

    // println!("s after taken: {}", s); // s is no longer valid
    
    let x = 5; // x comes into scope
    makes_copy(x); // x moves into function, but i32 is Copy
                  // x still usable

    println!("integer still usable: {}", 5);

    let s1 = gives_ownership(); // moves return value into s1
    println!("gives fn: {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_ownership(s2); // s2 moved into fn and gives it to s3
                                            // s2 no longer valid

    //println!("gave ownership: {}", s2); // s2 value was dropped
    println!("took ownership: {}", s3);

    // another take and give using tuple return //
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("length of {}, is {}", s2, len);
    // println!("{}", s1); // no longer valid



}

fn takes_ownership(some_string: String) {
    println!("takes fn: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("copies fn: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // return value
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string // return
}

fn calculate_length(s: String) -> (String, usize) {
    let lenght = s.len(); 
    (s, lenght) // return a tuple
}
