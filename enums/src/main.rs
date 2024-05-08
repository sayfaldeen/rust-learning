use std::default;

//#################### Import modules ####################//
use colored::Colorize;

fn _sec(title:&str){
    println!("\n{title}");
    println!("{}", "=".repeat(title.len()))
}

// Set up example enum for IP addresses
#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// Set up example message enum
#[derive(Debug)]
enum Message {
    Info(String),
    Warn(String),
    Error(String)
}

impl Message{
    fn call(&self){
        match self {
            Message::Info(message) => println!("{}", message.green()),
            Message::Warn(message) => println!("{}", message.yellow()),
            Message::Error(message) => println!("{}", message.red())
        }
    }
}

// We can also write using structs for to add information to the inputs expected
//enum message {
    //quit,
    //write {text:string},
//}


// Create a function to add 5 to any number
fn plus_five(x: Option<i32>) -> i32 {
    5 + x.unwrap_or(0)
    // The unwrap_or function allows to replace the `None` with a default value
        // this allows us to actually work with Optional data
}

#[derive(Debug)]
enum CoinType {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl CoinType {
    fn value(&self) -> f64 {
        match self {
            CoinType::Penny => 0.01,
            CoinType::Nickel => 0.05,
            CoinType::Dime => 0.10,
            CoinType::Quarter => 0.25
        } // end match

    } // end value()

} // end impl

#[derive(Debug)]
struct Coin {
    coin_type: CoinType,
    state: String,
    year: u32,
    rare: bool
}

impl Coin {
    fn value(&self) -> f64 {
        if self.rare {4.0 * self.coin_type.value()}  else {self.coin_type.value()}
    }
}


fn main() {

    _sec("Basic enums");
    let v4_ip = IPAddr::V4(127,0,0,1);
    let v6_ip = IPAddr::V6(String::from("126.0.0.1"));

    println!("{:?} {:?}", v4_ip, v6_ip);

    _sec("Adding variants and methods to enums");
    // Instantiate the message
    let info_message = Message::Info(String::from("This is info"));
    //println!("{:?}", message);
    info_message.call();

    let warn_message = Message::Warn(String::from("This is a warning"));
    warn_message.call();

    let err_message = Message::Error(String::from("This is an error"));
    err_message.call();

    _sec("Using the `Optional` enum");
    let some_num = Some(5);
    let some_string = Some(String::from("Hello"));
    let absent_num: Option<i32> = None; // must specify the type for None; cannot be inferred

    println!("{:?} -- {:?} -- {:?}", some_num, some_string, absent_num);

    // Add together an optional value and i32; different types so they can't be added normally
    let x = Some(2);
    let result = plus_five(x);
    println!("5 + {:?} = {}", x, result);

    let x = None;
    let result = plus_five(x);
    println!("5 + {:?} = {}", x, result);

    _sec("Advanced enums with multiple variants and varying information");
    let coin = Coin{coin_type:CoinType::Quarter, state:"Florida".to_string(), year:1992, rare:true};
    println!("{:?}, value = USD {}", coin, coin.value())

}
