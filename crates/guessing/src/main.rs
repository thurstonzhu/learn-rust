use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // returns a `Result` (`Ok` or `Err`)
            .expect("Failed to read line"); // error handling

        // NOTE: this works
        // let guess: u32 = guess.trim().parse() // returns a `Result` (`Ok` or `Err`)
        //     .expect("Please type a number!"); // error handling

        // NOTE: this compiles but does not set `guess` properly
        // match guess.trim().parse() { // returns a `Result` (`Ok` or `Err`)
        //     Ok(num) => {
        //         println!("Succesfully parsed {num} into u32");
        //         guess = num
        //     }
        //     Err(_) => {
        //         println!("That's not a valid number!");
        //     }
        // }

        // NOTE: this works but doesn't let you log or do anything in the success case
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("parse: That's not a valid number!");
        //         return; // Exit early
        //     }
        // };

        // NOTE: this one is best
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                // println!("parse: Successfully parsed \"{num}\" into u32");
                num // Return the parsed number
            },
            Err(_) => {
                println!("parse: That's not a valid number!");
                continue; // continue rather than return out of main()
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
