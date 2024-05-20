use std::fs;
use std::process;
use std::error::Error;
use regex::Regex;
use clap::Parser;

// Create a struct for storing the params
#[derive(Parser, Debug)]
struct Params {
    #[clap(short, long)]
    filename: String,

    #[clap(short, long)]
    query: String
}

#[derive(Debug, PartialEq)]
struct FileContents {
    contents: String,
}

impl FileContents {

    fn read_file(filename: &str) -> Result<FileContents, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        Ok(FileContents{ contents })
    } // end read_file

    fn find_in_file_sensitive(&self, query:&str) -> Vec<&str> {
        let mut results = Vec::new();

        for line in self.contents.lines() {
            if line.contains(&query) {results.push(line)}
        }
        
        results

    } // end find_in_file_sensitive()

    fn find_in_file_insensitive(&self, query:&str) -> Vec<&str> {
        let mut results = Vec::new();

        for line in self.contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }
        results
    } // end find_in_file_insensitive()

    fn find_whole_word_in_file(&self, query: &str) -> Vec<&str> {
        let pattern_string = format!(r"\b{}\b", regex::escape(query));
        let pattern = Regex::new(&pattern_string).unwrap();
        let mut results = Vec::new();

        for line in self.contents.lines() {
            if pattern.is_match(line) {results.push(line)}
        }

        results
    } // end find_in_file_whole_word

}

fn main() {
    // Take in the params
    let params = Params::parse();

    // Read the file contents and handle possible errors
    let file_contents = FileContents::read_file(&params.filename).unwrap_or_else( |err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    println!("\nfilename: {} -- query: {}", params.filename, params.query);
    
    println!("\nSensitive match");
    let results = file_contents.find_in_file_sensitive(&params.query);
    for result in results {
        println!{">{}", result};
    }

    println!("\nInsensitive match");
    let results = file_contents.find_in_file_insensitive(&params.query);
    for result in results {
        println!{">{}", result};
    }

    println!("\nMatch whole-word");
    let results = file_contents.find_whole_word_in_file(&params.query);
    for result in results {
        println!{">{}", result};
    }

}

// Write some tests for the functions
#[cfg(test)]
mod tests {
    use crate::FileContents;

    #[test]
    fn test_read_file(){
        let fc = FileContents::read_file("./poem.txt").unwrap();

        let _contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
";

        assert_eq!(fc.contents, _contents)

    } // end test_read_file()

    #[test]
    fn test_sensitive_match() {
        let fc = FileContents::read_file("./poem.txt").unwrap();
        let left = vec!["Are you nobody, too?"];

        assert_eq!(left, fc.find_in_file_sensitive("Are"))

    } // end test_sensitive_match

    #[test]
    fn test_insensitive_match() {
        let fc = FileContents::read_file("./poem.txt").unwrap();
        let left = vec!["I'm nobody! Who are you?", "Are you nobody, too?"];

        assert_eq!(left, fc.find_in_file_insensitive("Are"));
    }

}
