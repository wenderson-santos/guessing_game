use std::io; // To obtain user input and then print the result as output.

// Rng trait defines methods that random number generators implement,
// and this trait must be in scope for us to use those methods.
use rand::Rng;

// Entry point into the program
fn main() {
    // println!() is a macro that prints a string to the screen
    println!("Guess the number!");

    // gives us the particular random number generator we're going to use
    // one that is local to the current thread of execution and is seeded by the OS
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

    // let statement to create the variable
    // variable to store the user input
    let mut guess = String::new();

    // allow us to handle user input
    io::stdin() // returns an instance of std::io::Stdin
        // get input from the user, the `&mut guess` tell what string to
        // store the user input
        .read_line(&mut guess) // & -> reference
        // read_line return an Result value
        // Result is an enumeration
        // Result's variants are Ok and Err.
        // Ok -> Successful
        // Err -> Operation Failed
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
