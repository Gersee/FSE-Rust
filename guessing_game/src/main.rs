extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut counter: u32 = 0;

    loop {
        counter = counter + 1;
        println!("Run #{} - Please input your guess.", counter);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This was not a number");
                counter = counter - 1;
                continue
            },
        };

        println!("You guessed: {}", guess);

        if match_guess(secret_number, guess) == true {
            println!("You win after {} guesses!", counter);
            break;
        }
    }
}

fn match_guess(secret_number: u32, guess: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less    => {
            println!("Too small!");
            return false;
        },
        Ordering::Greater => {
            println!("Too big!");
            return false;
        },
        Ordering::Equal => return true
    }
}
