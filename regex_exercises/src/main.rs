use regex::Regex;
use std::fs;
use std::process;

fn main() {

    // Read the file in as an example
    let contents = fs::read_to_string("./poem.txt").unwrap_or_else(|err|{
        eprintln!("Error reading file: {}", err);
        process::exit(1)
    });

    print!("\n{}", contents);

    // Set up pattern to match *body

    println!("\nFind any word that ends with 'body'");
    let pattern = Regex::new(r"\w*body").unwrap();

    // Find the first match
    let results = pattern.find(&contents).unwrap();
    println!("\nFind first match only");
    println!("{:#?}", results);

    // Find all matches
    let mut results = Vec::new();

    for i in pattern.find_iter(&contents){
        results.push(i)
    }
    println!("\nFind all matches");
    println!("{:#?}", results);

}


//#################### Write some tests ####################//
#[cfg(test)]
mod tests {
    use super::*; // use everything within this file

    #[test]
    fn test(){}

}
