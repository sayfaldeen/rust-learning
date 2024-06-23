/*
- This module is to play around a little more with options and results
 */
use std::process;

fn main() {
    let username = get_username(0).unwrap_or_else(||{
        eprintln!("Error getting username");
        process::exit(33)
    });

    println!("Username: {username}")
}

// Use options when we can either return a result or return nothing
fn get_username(user_id:u32) -> Option<String> {

    let mut query = String::new();

    // Create the query string
    if user_id == 0 {
        query.push_str(&format!("select username from user_db where user_id == {user_id}"))
    }

    // Query the DB
    match query_db(query) {
        Ok(res) => Some(res),
        Err(..) => None
    }

}

// Use results when we can either return a success or error
fn query_db(query:String) -> Result<String, String> {
    if query.is_empty() {
        Err("Query string is empty!".to_string())
    }
    else {
        Ok("Ferris".to_string())
    }
}
