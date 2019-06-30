use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("GUESS THE NUMBER!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("INPUT A GUESS (NUMBER): ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() { // parse returns ok or err
            Ok(num) => num, 
            Err(_) => { // _ catch all

                println!("Type a number!!!...\n");
                continue;
             } 
              
        };

        println!("YOU GUESSED: {}", guess);


        //  match checks the arms patterns 'Ordering::xxx' 

        match guess.cmp(&secret_number) { // cmp returns Ordering::Enum enum could be <, > or =
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("yeah!");
                break;
            }
        }

    }
}