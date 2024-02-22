fn main() {
    println!("\n");
    other_func(10);

    println!("\n");
    print_labeled_measure(32.3, "mL");

    println!("\n");
    using_blocks(20);

    println!("\n");
    let x = minus_two(6);
    println!("minus_two(6) returns {x}")
}

fn other_func(x: i32) {
    println!("2x = {}", 2 * x);
}

fn print_labeled_measure( val:f64, units:&str ) {
    println!("The measurement was {val} {units}");
}

fn using_blocks(x:i32) {
    //let r = (let y = x+1); //cannot assign a value using another value like so

    // We can correct this using a block
    let _y = {
        let r = x;
        r + 2
    };

    // If we add a ; this would turn the expression into a statement and break the code
    // since statements don't return anything
    //let y = {
        //let r = x;
        //r + 2;
    //};
    
    // The below code would also work just fine
    let r = x;
    let y = r+2;

    println!("y = {y}")
}

fn minus_two(x:i32) -> i32 {
    x - 2
}
