use std::fmt::Display;

pub struct Article {
    author: String,
    title: String,
    content: String
}

pub struct Tweet {
    username: String,
    content: String,
    reply_count: i32,
    retweet_count: i32
}

pub trait Summary {
    //fn summarize(&self) -> String; <- this is without a default implementation; each struct has to define own

    fn summarize(&self) -> String {
        "Read more...".to_string()
    }
}

// Implement Summary trait for Artcile struct
impl Summary for Article {} // this uses the default implementation
    //fn summarize(&self) -> String {
        //format!("{} by {}", self.title, self.author)
    //}

// Implement Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        let s = if self.content.len() > 20 {
            format!("{}...", &self.content[0..20])
        } else {format!("{}", &self.content)};

        format!("{} by @{}", s, self.username)
    }
}

// Create a function that only takes in structs that implement Summary
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T
}

impl <T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self {x , y}
    }
}

impl <T> Pair<T>
    where T: Display + PartialOrd
{
    fn largest(&self) -> &T {
        if self.x >= self.y {&self.x} else {&self.y}
    }
}

fn main() {

    let article = Article { 
        author: "John Doe".to_string() , title: "The sky is falling!".to_string(), 
        content: "Read more...".to_string()
    };

    let tweet = Tweet{
        username:"johndoe".to_string(), content:"This is just a short test tweet".to_string(),
        reply_count: 3, retweet_count: 0
    };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());

    notify(&article);

    let pair = Pair::new(3, 6);
    println!("The largest number in the pair {:?} is {}", pair, pair.largest());
}
