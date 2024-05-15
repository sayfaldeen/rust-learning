// This module will focus on generic lifetime annotations

fn main() {
    let s1 = "string1";
    let s2 = "stringg2";

    let longest = longest(s1, s2);

    println!("The longest string is {longest}");


    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split(".").next().expect("No more sentences found");
    let excerpt = ImportantExcerpt{part: first_sentence};
    println!("{:?}", excerpt);
}


// Need to specify lifetimes here because we are using different input args into a function and they are both references
fn longest<'a>(s1:&'a str, s2:&'a str) -> &'a str {
    if s2.len() > s1.len() {s2}

    else {s1}
}


//#################### Define structs using lifetimes as well ####################//
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
