#![allow(dead_code)]
/*
FUNTION POINTERS

function pointers allow to use functions as arguments to other functions.
Functions are of type fn (do not confuse with closure trait Fn).
"fn" type is called a function pointer.

Funtion pointers implement all three of the closure traits (Fn, FnMut, FnOnce)
so that a function pointer can always be passed as an argument for a
function that expects a closure.
*/

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("do_twice: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) // instead of passing a closure
        // |i| i.to_string()
        .collect();
    for item in list_of_strings.iter() {
        println!("{}", item);
    }
}

// RETURNING CLOSURES // 

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
