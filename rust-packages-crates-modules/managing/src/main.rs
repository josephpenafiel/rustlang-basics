use std::collections::HashMap; // bringing HashMap into scope

// we can provide new names 

//use std::fmt::Result;
//use std::io::Result as ioResult;
use rand::Rng;

/*
this is ugly

use std::cmp::Ordering;
use std::io;

*/

//better 

use std::{cmp::Ordering, io};

//fn function1() -> Result {} // this returns a Result from the fmt module
//fn function2() -> ioResult<()> {} // returns a result from the io module

use std::collections::*; // bring all publics in collections into scope


fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(1,2);
    let secret_number = rand::thread_rng().gen_range(1,101);
}
