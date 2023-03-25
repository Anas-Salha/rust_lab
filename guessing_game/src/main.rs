use std::io;

fn main() {
    let game_name = "Guessing Game";
    let game_prompt = "Place your guess: ";
    println!("Welcome to {}!\n{}", game_name, game_prompt);

    let mut user_guess = String::new();
    
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

}
