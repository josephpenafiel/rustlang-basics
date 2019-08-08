/*
Unsafe Rust

when using the unsafe keyword the compiler allows to:

- dereference a raw pointer
- call an unsafe function or method
- Access or modify a mutable static variable
- implement an unsafe trait

different from references and smart pointers, raw pointers:
- are allowed to ignore the borrowing rules by having both immutable and
mutable pointers or multiple mutable pointers to the same location
- aren;t guaranteed to point to valid memory
- are allowed to be null
- don't implement any automatic cleanup

*/

fn main() {
    let mut num = 5;

    // creating raw pointers by using as to cast an immutable and
    // a mutable reference into their corresponding raw pointer types.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // dereferencing the raw pointer
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
