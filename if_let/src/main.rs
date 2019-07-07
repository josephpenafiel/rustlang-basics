// CONCISE CONTROL FLOW WITH if let
/*
The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring 
the rest.
*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- more states --
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");


    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three!"),
        _ => (),
    }

    /*
    We want to do something with the Some(3) match but do nothing with any other Some<u8> value or the None value. To satisfy 
    the match expression, we have to add _ => () after processing just one variant, which is a lot of boilerplate code to add.
    */

    let some_u8_value = Some(3u8);

    if let Some(3) = some_u8_value {
        println!("three!");
    }

    // CODE WITH MATCH 

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => {
            println!("[MATCH] State quarter from {:?}", state);
        },
        _ => count += 1,
    }

    println!("count {}", count);

    // CODE WITH IF LET

    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("[IF-LET] State quarter from {:?}", state );
    } else {
        count += 1;
    }

    println!("count {}", count);

}
