/*

   - This exercise code will work to expand on previous sections where we worked with module trees
   - The code for the different collections will each be stored in a different file
     - These files will then be called into scope and used to execute the code

*/

// Bring neccessary modules into scope
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

mod modules;

fn main() {

    //#################### Start vector exercises ####################//
    modules::_sec("Vectors examples");

    let empty_vec = modules::vectors::make_new_empty_vector();
    let mut vec1 = empty_vec;
    vec1.push(0);
    vec1.push(1);
    vec1.push(2);

    println!("{:?}", vec1);

    let mut vec2 = modules::vectors::make_non_empty_vector();
    println!("{:?}", vec2);

    // Access the elements in the vector
    let third = vec2[2];
    // let third = &vec2[2]

    vec2.push(99);
    println!("{third}"); // this works because `third` is not defined using a reference

    // Handling out-of-bound index errors in vectors
    let index_100 = match vec2.get(100) {
            Some(val) => Some(val),
            None => {println!("No value was found"); None}
    };

    println!("index_100 = {:?}", index_100);

    // We can mutate a vector in place if we would like
    for i in &mut vec2 {
        *i += 100;
    }

    // We can also iterate through the vector using a for-in loop
    //let mut n = 0;
    //for i in &vec2 { // if we don't reference here, the ownership will be moved and we can't print at L45
        //println!("{n}) {i}");
        //n+=1
    //}

    modules::vectors::print_vec_contents(&vec2);

    println!("{:?}", vec2); // used to be L45; might move so I commented it here

    // You can conditionally return values from a vector as well
    enum DataCellType {
        IntData(i32),
        FloatData(f64),
        StringData(String)
    }
    
    // Return a value only if it is DataCellType::IntData
    let row = vec![
        DataCellType::IntData(10),
        DataCellType::FloatData(10.1),
        DataCellType::StringData("blue".to_string())
    ];

    // Iterate through the row vector and look at the types
    let mut n = 0;
    while n < row.len() {
        match row[n] {
            DataCellType::IntData(int) => println!("data at col {n} is an integer: {int}"),
            _ => println!("Data at col {n} is not an integer")
        };
        n+=1;
    } // end match


    //#################### Start string exercises ####################//
    modules::_sec("String exercises");
    
    let empty_string = modules::strings::make_new_empty_string();
    println!("empty string: {empty_string}");

    let new_string = modules::strings::make_string_from_str_slice("test string");
    println!("new string: {new_string}");

    // Manipulate empty string and add to it; use borrowing to be rid of new_string after push
    let mut mod_string = empty_string;
    mod_string.push_str("This string has been modified");
    println!("modified string from empty: {mod_string}");

/*
 *    let cat_string = new_string + " + " + &mod_string; // needs to be a reference here; concatting strings works with
 *                                                // owned + borrowed pattern only
 *    println!("concattenated string: {cat_string}");
 */

    //println!("new_string: {new_string}"); // does not work because `+` takes ownership of the left-side string
    // We can concat strings without taking ownership though using `format!()` macro
    let cat_string = format!("{} + {}", new_string, mod_string);
    println!("cat_string: {cat_string}");
    //println!("mod_string: {mod_string}"); works because no ownership taken

    // String contents can be accessed in mutliple ways
    let s = "string".to_string();

    // Bytes can also be accessed
    println!("\nAccessing strings using bytes");
    for (n,b) in s.bytes().enumerate(){
        println!("{n}) {b}");
    }

    // Chars are equivalent to letters in english since english letters are one-byte encoded
    println!("Accessing string using chars");
    for (n,c) in s.chars().enumerate() {
        println!("{n}) {c}");
    }

    // We can also access the grapheme clusters using a crate; unicode-segmentation
    println!("Accessing string using grapheme clusters");
    for (n,g) in s.graphemes(true).enumerate(){
        println!("{n}) {g}");
    }


    //#################### HashMaps section ####################//
    modules::_sec("HashMaps exercises");
    
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    hashmap.insert("blue".to_string(), 20);
    hashmap.insert("yellow".to_string(), 10);
    println!("{:#?}", hashmap);

    let blue_score = hashmap.get("blue");
    println!("The score for blue is {:?}", blue_score);

    // We can also conditionally add to the hashmap using entry().or_insert() method
    hashmap.entry("yellow".to_string()).or_insert(100); // unchanged; it already exists
    hashmap.entry("black".to_string()).or_insert(99); // added, did not previously exist

    println!("\nIterate through the key-value pairs in a hashmap");
    for (k,v) in hashmap {
        println!("{k}: {v}");
    }

    let word_counts = modules::hashmaps::count_word_occurrences("hello world. hello again world");
    println!("{:#?}", word_counts);

} // end main
