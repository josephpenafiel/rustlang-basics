use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

#[allow(unused_variables)]
fn main() {
    println!("Hello, error handling!");
    // UNRECOVERABLE ERRORS panic!

    /*
    When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.
    */

    //panic!("false alarm!") // stops execution

    //let v = vec![1,2,3];

    //v[20]; // panic! 

    // running a program with RUST_BACKTRACE=1 will show a list of function calls that lead to the error

    // RECOVERABLE ERRORS with Result<T, E>
    /*
    the Result enum is defined as having two variants, Ok and Err, as follows:

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    T represents the type of the value that will be return in the case of success
    E represents the type of the error that will be return in a failure

    */

    let f = File::open("hello.txt"); // returns a Result<File, Err>

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("created {:?}", fc);
                    fc
                },
                Err(e) => panic!("there was a problem creating the file {:?}", e)
            }
            other_error => panic!("there was a problem")
        }
    };

    /*
    the error inside of Err variant is an io::Error, which is a struct provided by the 
    standard library. The struct has a method called kind that returns a io:ErrorKind value.
    ErrorKind is an enum provided by the std lib and has variants that represents the different
    kinds of errors that might occur.
    */

    // this is better // 

    let f = File::open("hello_better.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_better.txt").unwrap_or_else(|error| {
                panic!("there was a problem creating the file {:?}", error);
            })
        } else {
            panic!("there was a problem opening the file: {:?}", error);
        }
    });

    /*
    the code above doesn't use the match expressions. Instead uses unwrap() which returns the value
    inside the Ok(T) in the Result enum. If  the Result is Err, unwrap calls panic.
    */

    // let f = File::open("no_exist.txt").expect("Failed!"); // file doesnt exist. Execution stops 

    /* the expect method is similar to unwrap, but let's us pick the panic! message. */

 







}


   // PROPAGATING ERRORS 

/*
when writing a function whose implementation calls something that might fail, instead of handling the error 
within the function, we can return the error to the calling code so that it can decide what to do. 
this is known as propagating the error and gives more control to the calling code, where there migt be more information
or logic that dictates how the error should be handled than what you have available in the context of the code.
*/



fn read_username_from_file() -> Result<String, io::Error> { // returns a Result<T, E> which have been filled with String, and io::Error types
    let f = File::open("hello.txt");

    let mut f = match f {

        Ok(file) => file, 
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// A SHORTCUT TO PROPAGATING ERRORS

fn read_username_from_file_propagate() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // the ? operator does the same as the match in the code above (can only be used in the Result enum)
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // note there's no semicolon
}

// even shorter

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    //or 

    /*
    use std::fs 

    fs::read_to_string("hello.txt");
    */
}