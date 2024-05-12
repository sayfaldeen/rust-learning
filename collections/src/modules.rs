/*

   - This file will contain three modules to be called and used in main.rs

   1. vectors
   2. strings
   3. hashmaps

*/

use std::collections::HashMap;

pub fn _sec(title:&str) {
    println!("\n{title}");
    println!("{}", "=".repeat(title.len()))
}

pub mod vectors {

    pub fn make_new_empty_vector() -> Vec<i32> {
        Vec::new() // create a new and empty vector
    } // end make_new_empty_vector()

    pub fn make_non_empty_vector() -> Vec<i32> {
        vec![0,1,2,3,4,5]
    }

    pub fn print_vec_contents(v:&Vec<i32>) {
        let mut n = 0;

        for i in v {
            println!("{n}) {i}");
            n+=1
        } // end for-in loop
    } // end print_vec_contents

} // end vectors module


pub mod strings {

    pub fn make_new_empty_string() -> String {
        String::new()
    } 

    pub fn make_string_from_str_slice(s:&str) -> String {
        s.to_string()
    }

} // end strings module


pub mod hashmaps {
    use std::collections::HashMap;

    pub fn count_word_occurrences(s:&str) -> HashMap<&str, i32> {
        let mut count_map = HashMap::new();

        for word in s.split_whitespace() {
            let count = count_map.entry(word).or_insert(0); // if it does not exist make zero, else pass
            *count += 1 // add one to the count; occurs in the hashmap
        }

        count_map

    } // end count_word_occurrences()


} // end hashmaps mod
