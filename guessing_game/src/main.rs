// Guessing game exercise

// Bring necessary modules into scope
use std::io; // input/output library
use rand::Rng; // random number generator library
use std::cmp::Ordering; // library for comparing ordering

// Create the main function
fn main() {
    
    // Set up the secret number
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Guess the number");

    loop {
        println!("Please input your guess.");

        // Set up variable to hold the input
        let mut guess = String::new(); // new mutable string variable; immutable by default

        // store the input in `guess` as a reference (&); default is immutable too
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // if an error occurs, return the string


        // Turn the string variable into numeric; input will always be string
        // Using `match` allows us to perform logic and inform user to provide a correct guess without breaking the code
        let guess: u32 = match guess
            .trim()  // remove whitespaces and newlines
            .parse() { // turn the stirng into what it has been typed to (u32 here)
                Ok(num) => num,
                Err (_) => {
                    println!("{guess} is not a number. Please provide a number"); continue;
                },
            };

        // Compare the strings
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess was too small"),
            Ordering::Equal => {
                println!("You're right!");
                break;
            }, // end ordering::equal
            Ordering::Greater => println!("Guess was too big")
        } // end guess

        // print the guess
        println!("You guessed: {guess}")
    
    } // end loop

} // end main
