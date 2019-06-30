fn main() {
    
    println!("Hello, world!");
    let number = 3; 

    // IF Statements

    if number < 5 {
        println!("true");
    } else {
        println!("false"); 
    }

    //this won't work
    /*
    if number { // if statements must evaluate a boolean
        logic;
    }
    */

      if number % 5 == 0 {
        println!("divisible to 5");
    } else if number % 3 == 0{
        println!("divisible to 3"); 
    } else {
        println!("not divisible to 5 or 3")
    }

    // using if in a let statement

    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six" // won't work, must be same type
    };


    // LOOPS 

    // loop {
    //     println!("this is a loop");
    // }

    // Returning value from loops 

    let mut counter = 0; 

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    }; // bind counter value to result variable

    println!("counter = {}, result = {}", counter, result);

    // conditional Loop

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // LOOP through array

    let array = [10,20,30,40,50];

    let mut index = 0;

    while index < 5 {
        println!("array value at {} is {}", index, array[index]);
        index += 1;
    }

    //better 

    for element in array.iter() {
        println!("value {}", element);
    }

    // Another way of for loops

    for number in (1..4).rev() { // (1..4) = 1,2,3,4
        println!("{}!", number);
    }

    println!("Go!");


}
