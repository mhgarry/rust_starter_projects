/*
 * Create a program that will generate a random number between 1 and 100.
 * When the program runs, it will prompt the user to enter a guess.
 * Upon entering a guess, the game will indicate whether the guess is too low or too high.
 * If the guess is correct the game will print a congratulatory message and exit.
*/

use std::io;

fn main() {
    println!("Guess a number between 1 and 100!");
    println!("Please enter your guess:");

    let mut guess = String::new(); // mut indicates a mutable variable in Rust (default is immutable) for dynamic user input.

    io::stdin()
        .read_line(&mut guess) // read_line takes the user input and stores it in the guess variable.
        .expect("Failed to read line"); // expect is a method that handles errors. If an error occurs, it will print the message provided.

    println! {"You guessed: {}", guess}; // print the user's guess.
}
