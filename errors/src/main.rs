//#################### Call necessary modules into scope ####################//
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    //panic!("Crash and burn"); // graceless panic

    // Read file
    //let mut f = File::open("test-hello.txt")?; // The `?` symbol is a special symbol
            /*
             - Operates similar to the .unwrap() method but instead of panicking, returns the Err
             - This allows us to propagate the errors and gracefully handle them
             */
    let file_data = return_file_as_string("test-hello.txt");
    println!("{:?}", file_data);


}

pub fn return_file_as_string(path:&str) -> Result<String, io::Error> {
    //let mut f = File::open(path)?;
    //let mut data = String::new();
    //f.read_to_string(&mut data)?;
    //Ok(data)

    // All of the above code can be condensed by chaining; you save a few lines
    let mut data = String::new();
    File::open(path)?.read_to_string(&mut data)?;
    Ok(data)

    /*
     - The `?` symbol is a special symbol
     - Operates similar to the .unwrap() method but instead of panicking, returns the Err
     - This allows us to propagate the errors and gracefully handle them
     */


}

fn _old_open_file_example(){
    // We can handle the result enum using a match expression
    let _f = match File::open("test-hello.txt") {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("test-hello.txt") {
                Ok(fc) => fc,
                Err(e2) => panic!("Could not create new file: {:?}", e2)
            }
            other_error => panic!("Could not open file: {:?}", other_error)
        }
        //Err(e) => panic!("Problem reading file {:?}", e)
         /*
          - e: { code: 2, kind: NotFound, message: "No such file or directory" }
          - This shows that there are error codes, kinds, and associated messages
            - This additional info can be further used for matching
          */
        
    };

}
