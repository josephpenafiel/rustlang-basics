fn main() {
    println!("Hello, world!");
    /*
    Another data type that does not have ownership is the slice. Slices let you reference a contiguous 
    sequence of elements in a collection rather than the whole collection.

    Here’s a small programming problem: write a function that takes a string and returns the first word 
    it finds in that string. If the function doesn’t find a space in the string, the whole string must 
    be one word, so the entire string should be returned.
    */

    let mut s = String::from("hello world");
    let word = first_word(&s);
    
    println!("last index of first word: {}", word);

    s.clear(); 

    println!("word still usable {}", word);

    /*
    This program compiles without any errors and would also do so if we used word after calling s.clear().
    Because word isn’t connected to the state of s at all, word still contains the value 5. We could use 
    that value 5 with the variable s to try to extract the first word out, but this would be a bug because 
    the contents of s have changed since we saved 5 in word.
    */

    // === STRING SLICES === //

    /*
    A string slice is a reference to part of a String
    */

    let s = String::from("hello world");

    let hello = &s[..5]; // contains a ptr address from index 0 to 5 of s
              //  [0..5]  same as above
    let world = &s[6..]; // contains a ptr address from 6th element to the end
              //  [6..len] same as above 

    println!("hello slice: {}", hello);
    println!("world slice: {}", world);

    /* 
    re-doing the problem stated at the beginning
    */
    let s = String::from("hello world");
    let first_word = better_first_word(&s);

    println!("first word: {}", first_word);

   // s.clear(); // this won't compile

   // === STRING LITERALS ARE SLICES === //

   /*
   the type of s is &str. It's a slice pointing to a specific point of the binary. 
   This is why string literals are immutable; &str is an immutable reference.

   let s = "hello, world";  // a slice of type &str
   */

   // === STRING SLICES AS PARAMETERS === // 

    let my_string = String::from("hello world");

    println!("my_string: {}", my_string);

    let word = best_first_word(&my_string[..]); // can pass a slice of a &String

    println!("&String word: {}", word);

    let my_string_literal = "hello world";

    println!("my_string_literal: {}", my_string_literal);

    let word = best_first_word(&my_string_literal[..]); // can pass a &str
                            //(my_string_literal); // works too because the var is of type &str

    println!("&str word: {}", word);

    // === OTHER SLICES === // 

    let a = [1,2,3,4,5];
    
    let slice = &a[1..3];

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string into array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iter makes an iterator
                                                 // enumerate return a tuple 
        if item == b' ' { // looks for a space
            return i; // return the index at which the first word ends
        }
    }
    s.len() // if there are no spaces return the whole length of the strin
}

fn better_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // return the whole word
}

fn best_first_word(s: &str) -> &str { // this allows us to use the same function on &String and &str values
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // return the whole word
}
