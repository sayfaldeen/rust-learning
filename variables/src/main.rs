fn main() {
    variable_scope();
    
    let i = 10;
    let j = 2;
    let z = add_two_numbers(i,j);

    println!("z = {}", z);
}

fn variable_scope() {
    let mut x: u32 = 1_000;
    println!("x = {x}");

    x = 100;
    println!("Mutated x = {x}");

    let x = 10;
    println!("The 'shadowed' x = {x}");
}

fn add_two_numbers(x:i32, y:i32) -> i32 {
    println!("{x} + {y} = {}", x+y);
    x + y // statements end with ';' and do not return data
            // but blocks '{}' can work as functions and return values
}
