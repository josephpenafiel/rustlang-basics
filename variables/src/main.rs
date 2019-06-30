fn main() {

    // === won't work === //
    // let x = 5;
    // println!("The value of x is: {}", x);
    
    // x = 6; // error! 
    // println!("The value of x is: {}", x);

    
    // === will work === //
    let mut x = 5; // has mut modifier
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);


    /* 
    CONSTANTS
    Constants can be declared in any scope, including the global scope, which 
    makes them useful for values that many parts of code need to know about.
    Constants may be set only to a constant expression, not the result of a 
    function call or any other value that could only be computed at runtime. */
    
    const MAX: u32 = 100_000; // declaring a constant 
    println!("Const value is {}", MAX);

    /*
    SHADOWING
    you can declare a new variable with the same name as a previous variable, 
    and the new variable shadows the previous variable. Rustaceans say that 
    the first variable is shadowed by the second, which means that the second 
    variable’s value is what appears when the variable is used. We can shadow 
    a variable by using the same variable’s name and repeating the use of the 
    let keyword as follows:
    */

    let x = 10;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is: {}", x); // 22

    /*
    Shadowing is different from marking a variable as mut, because we’ll get 
    a compile-time error if we accidentally try to reassign to this variable 
    without using the let keyword. By using let, we can perform a few 
    transformations on a value but have the variable be immutable after those 
    transformations have been completed.

    The other difference between mut and shadowing is that because we’re 
    effectively creating a new variable when we use the let keyword again, 
    we can change the type of the value but reuse the same name
    */

    let spaces = "     ";
    println!("Spaces: {}...", spaces); // prints spaces
    let spaces = spaces.len();
    println!("Spaces: {}", spaces); // prints a number

    // === this won't work === //
    // let mut spaces = "     ";
    // spaces = spaces.len(); // can't change variable's type without shadowing




    

}
