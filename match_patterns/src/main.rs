 // === MATCH Control Flow === //

    /*
    Rust has an extremely powerful control flow operator called match that allows you to compare a value against a series 
    of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable 
    names, wildcards, and many other things.
    Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes 
    along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through 
    each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be 
    used during execution.
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

fn main() {
    println!("Hello, world!");
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alabama);

    let one = value_in_cents(penny);
    let twenty_five = value_in_cents(quarter);
    println!("penny is {} cent", one );
    println!("quarter is {} cents", twenty_five);

    // the Options<T> match 

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six {:?}", five, six );

    /*
    MATCHES are EXHAUSTIVE
    this won't work 
    fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // here should've been a None case
        }
    }
    */

    // === THE _ PLACEHOLDER === //
    /*
    Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255. 
    If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255. Fortunately, 
    we don’t have to: we can use the special pattern _ instead:
    */

    let some_u8_value = 5u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // do nothing for the rest
    }
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // match arms
        //Pattern        Code
        Coin::Penny   =>  {
            println!("lucky penny!");
            1
        },
        Coin::Nickel  =>  5,
        Coin::Dime    =>  10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state );
            25
        } 
    }
}

// === MATCHING WITH Option<T>

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


