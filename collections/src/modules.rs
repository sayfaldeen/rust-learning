/*

   - This file will contain three modules to be called and used in main.rs

   1. vectors
   2. strings
   3. hashmaps

*/

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

}
