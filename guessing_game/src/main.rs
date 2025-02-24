/*
 * Create a program that will generate a random number between 1 and 100.
 * When the program runs, it will prompt the user to enter a guess.
 * Upon entering a guess, the game will indicate whether the guess is too low or too high.
 * If the guess is correct the game will print a congratulatory message and exit.
*/

use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100!");
    println!("Please enter your guess:");

    // mut indicates a mutable variable in Rust (default is immutable) for dynamic user input.
    let mut guess = String::new();

    // generate a random secret number between 1 and 100 that the user will try to guess.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // print the secret number to console for testing
    println!("The secret number is: {}", secret_number);

    io::stdin()
        .read_line(&mut guess) // read_line takes the user input and stores it in the guess variable.
        .expect("Failed to read line"); // expect is a method that handles errors. If an error occurs, it will print the message provided.

    println! {"You guessed: {}", guess}; // print the user's guess.
}
