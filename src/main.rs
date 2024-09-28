use std::io; // To obtain user input and then print the result as output.

// Entry point into the program
fn main() {
    // println!() is a macro that prints a string to the screen
    println!("Guess the number!");

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
