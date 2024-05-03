use colored::Colorize;
use std::io;

fn main() {

    _sec("conditional statements");
    // Take in user input
    println!("Check if 'a' is in provided string: ");
    let input = take_user_input();
    conditional_statements(input);

    _sec("condition logic cont.");
    // Take in numeric input
    println!("Check if provided number is even: ");
    let input = loop {
        let input = take_user_input();
        match input.trim().parse() {
            Ok(num) => {
                break num
            },
            Err(_) => {
                println!("{}", "Please provide numeric input".red());
                continue
            }
        };
    };
    let even = check_if_even(input);
    let example = if even {2} else {1};
    println!("{} -- An example of a similar number is {}", even, example);

    //#################### Show regular loops ####################//
    // Let's count up throughout the loop
    let mut counter = 0;
    loop {
        if counter == 10 {
            println!("Counting is done! {counter}");
            break
        }
        else {
            counter += 1
        }
    }

    // We can also return the loop as a result; like a block
    let mut counter_2 = 0;
    let result = loop {

        if counter_2 == 100_000 {
            break counter_2
        }

        else {
            counter_2 += 1;
            //println!("\rCounting ...");
            //print!("\x1B[2K");
        }
    };

    print!("Second counter: {counter_2} {result}");

    //#################### Show while loops ####################//
    _sec("While loops");
    let mut while_var = 12;
        while while_var % 11 != 0 {
            while_var += 1;
        };
    println!("while_var % 11 != 0: {while_var}");

    //#################### Show for-in loops ####################//
    _sec("For-in loops");
    let mut arr: [i32; 6] = [0, 10, 20, 30, 40, 50];
    let mut sum = 0;
    for i in arr.iter_mut() {
        *i += 10;
        sum += *i;
    };
    println!("{:?}: {sum}", arr);

    // Loop using a range expression; the `=` means inclusive
    for i in 1..=5 {
        println!("{i}");
    }

    
} // end main

fn _sec(title:&str){
   println!("\n{title}");
   println!("{}", "-".repeat(title.len()))
}

fn take_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn conditional_statements(input:String) {
    if input.contains("a") {
        let s = format!("{} '{}'", "There is an 'a' in", input.trim());
        println!("{}", s.green())
    }
    else {
        let s = format!("{} '{}'", "There is no 'a' in", input.trim());
        println!("{}", s.red())
            
    }
}

fn check_if_even(num:i32) -> bool {
    num % 2 == 0
}
