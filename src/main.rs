use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game! I will think of a number in the range [1, 100] and you'll try to guess it");

    let secret = rand::thread_rng().gen_range(0..=100);
    let mut attempts = 0;

    loop {
        println!("Give me your guess:");
        io::stdout().flush().expect("Couldn't flush stdout");

        let guess = match io::stdin()
            .lines()
            .next()
            .and_then(|v| v.ok())
            .and_then(|input| input.trim().parse::<i8>().ok())
        {
            Some(guess) if !(1..=100).contains(&guess) => {
                println!("Guess should be in the range [1, 100]");
                continue;
            }
            Some(guess) => guess,
            _ => {
                println!("Invalid guess, try again");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too low"),
            std::cmp::Ordering::Greater => println!("Too high"),
            std::cmp::Ordering::Equal => {
                println!("Found it in {attempts} guesses!");
                break;
            }
        }
    }
}
