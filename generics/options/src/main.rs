fn main() {
    if let Some(name) = get_username(1) {
        let name1 = Some(name)
    }

    println!("name1: {name1}");
}

fn get_username(id:u32) -> Option<String> {
    match id {
        1 => Some("Ferris".to_string()),
        _ => None
    }
}
