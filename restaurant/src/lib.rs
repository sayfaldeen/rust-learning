/*

- This module will house code for an example using Crate libraries to operate a restaurant

- The restaurant will be split up into two parts:
    1. The front: where hosting and serving occurs
    2. The back: where cooking occurs 

 */

//#################### Introduce needed modules into scope ####################//
use rand::{Rng, CryptoRng, Error};

// Set up module for front of house
mod front_of_house {

    // Modules can be nested; easier maintainability by grouping
    pub mod hosting { // needs to be made public so we can access it from outside code

        pub fn add_to_waitlist() {} // functions also have to be made public
        pub fn seat_at_table() {}

    } //end hosting

    pub mod serving {

        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}

    } //end serving

} // end front_of_house module

// Set up module for back_of_house
fn read_order() {}

mod back_of_house {
    use crate::front_of_house; // call front_of_house module into scope using absolute path

    pub fn cook_order() {
        super::read_order();
        front_of_house::serving::serve_order()
    }
    pub fn fix_incorrect_order() {}
}

// Set up a public function outside of front_of_house that uses some internal code
pub fn eat_at_restaurant() {
    // Call add_to_waitlist using absolute path as an example
    //crate::front_of_house::hosting::add_to_waitlist();

    // Call using relative path since this module is within this file
    front_of_house::hosting::add_to_waitlist();

}
