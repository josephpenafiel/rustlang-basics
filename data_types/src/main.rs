// DATA TYPES //

/*

INTEGERS

Length	Signed	 Unsigned
8-bit	  i8	    u8
16-bit	  i16	    u16
32-bit	  i32	    u32
64-bit	  i64	    u64
128-bit	  i128	    u128
arch	  isize	    usize

*/

// declaration 

fn main () {

let decimal: i32 = 45_456; // 45456
println!("An integer: {}", decimal);

let hex: i32 = 0xff;
let octal: i32 = 0o77;
let binary: i32 = 0b1111_0000;
let binary: u8 = b'A';
let number: u32 = "40".parse().expect("not a number!");
// this won't work
// let guess = "42".parse().expect("not a number"); // should give the variable a type

// FLOATS
/* 
f64 and f32
floats are f64 by default
*/ 

let float64 = 3.0; // by default is f64
let float32: f32 = 1.00;

// BOOLEANS

let boolean: bool = true;

// CHARACTERS
/*
Rust’s char type is four bytes in size and represents a Unicode Scalar Value, 
which means it can represent a lot more than just ASCII. Accented letters; 
Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are 
all valid char values in Rust. Unicode Scalar Values range from U+0000 to 
U+D7FF and U+E000 to U+10FFFF inclusive.
*/


let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';

// === COMPOUND TYPES === // 

// TUPLES 

let tuple: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tuple; // destructuring the tuple

let five_hundred = tuple.0; // accessing tuple's first member
let six_point_four = tuple.1; // 6.4
let one = tuple.2; // 1


// ARRAYS 

let array = [1,2,3,4,5]; // number array
let months = ["jan", "feb", "mar", "apr"]; // string array

let a:[i32; 5] = [1,2,3,4,5]; // define a 32bit int with 5 elements
let a = [3;5]; // [3,3,3,3,3]

//ACCESSING ARRAY MEMBERS 

let first = a[0];
let second = a[1];

//this won't work
// let out_of_bound = a[10];

}



