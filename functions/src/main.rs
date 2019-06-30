fn main() {
    println!("Hello, world!");

    //function call
    another_function(); // prints I'm a function
    function_with_args(4,5);
    let x = five();



}

//Function Definition 

fn another_function() {

    println!("I'm a function!");
}


//Parameters

fn function_with_args(x: i32, y: i32) {
    println!("x = {}, y = {}", x, y);
}

//  FUNCTION BODIES CONTAIN STATEMENTS AND EXPRESSIONS

/*
We’ve actually already used statements and expressions. Statements are 
instructions that perform some action and do not return a value. Expressions 
evaluate to a resulting value. Let’s look at some examples
*/

// a statement

//let y = 6;

//these won't work:
// let x = (let y = 6); // (let y = 6) doesn't return anything
// let x = y = 6; 

// expressions are:
/*
Calling a function
calling a macro
creating a scope {}
*/

// let y = {
//     let x = 3;
//     x + 1 // this line doesn't end with semicolon (it's a expression)
// }

// Functions with return values

fn five() -> i32 {
    5 // without ; telling implicitly to return 5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // note the lack of ; again (return the value)
}




