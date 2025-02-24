/*
 * Create a program that will generate a random number between 1 and 100.
 * When the program runs, it will prompt the user to enter a guess.
 * Upon entering a guess, the game will indicate whether the guess is too low or too high.
 * If the guess is correct the game will print a congratulatory message and exit.
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100!"); // print a message to the user.
    // add a loop to allow user to continue guessing.
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number between 1 and 100.

        println!("The secret number is: {secret_number}"); // print the secret number for debugging purposes.

        println!("Please enter your guess:"); // prompt the user to enter a guess.

        // mut indicates a mutable variable in Rust (default is immutable) for dynamic user input.
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess) // read_line takes the user input and stores it in the guess variable.
            .expect("Failed to read line"); // expect is a method that handles errors. If an error occurs, it will print the message provided.

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // parse the user input to an unsigned 32-bit integer.

        println! {"You guessed: {guess}"}; // print the user's guess.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"), // if the guess is less than the secret number, print "Too low!".
            Ordering::Greater => println!("Too high!"), // if the guess is greater than the secret number, print "Too high!".
            Ordering::Equal => println!("Congratulations! You guessed the secret number!"), // if the guess is equal to the secret number, print "Congratulations! You guessed the secret number!".
        }
    }
}
