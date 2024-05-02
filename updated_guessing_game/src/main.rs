// Import neccessary modules
use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {

    println!("\nThis is the updated guessing game!");

    // Let's start with generating a random number
    let rand_number = rand::thread_rng().gen_range(0..=100);
    println!("The random number is {rand_number}");

    loop {

        // Let's take in user-input
        let mut input:String = String::new();

        println!("What is your guess? ");
        io::stdin()
            .read_line(&mut input)
            .expect(&"Failed to read input".red().bold());

        // Cast the input from str to int
        let input: u16 = match input
            .trim() // remove whitespaces
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{} {}", input.trim().red().bold(), "is not numeric. Please provide numeric input\n".red());
                    continue
                } // end Err arm
            };

        // Check if the guess matches the random number
        match input.cmp(&rand_number) {
            Ordering::Less => println!("{}", "Number provided was too low!".yellow()),
            Ordering::Equal => {
                println!("{}", "You guessed right!".green().bold()); 
                break
            }, // end Equal arm
            Ordering::Greater => println!("{}", "Number you guessed was too high!".yellow())
        };
    }
}
