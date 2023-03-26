// Reference: The Rust Programming Language (online book)
// Ch2 - https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let game_name = "Guessing Game";
    let game_prompt = "Place your guess (1-100): ";
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_attempts = 5;

    println!("Welcome to {game_name}!");
    println!("You have {max_attempts} lives. Guess a randomly generated number (1-100)");


    for attempt in 0..max_attempts{
        let mut user_guess = String::new();
        println!("Attempt {} - {game_prompt}", attempt + 1);

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        if user_guess == "q\n"{
            println!("Quitter :).");
            return;
        }

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please select a number. Way to waste a life, buddy.");
                continue;
            }
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Git gud."),
            Ordering::Greater => println!("Too big! Git gud."),
            Ordering::Equal => {
                println!("Bingo! You guessed the correct number!");
                break;
            }
        }
    }
}
