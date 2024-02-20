fn main() {
    println!("\n");
    println!("Integers");
    integers();

    println!("\n");
    println!("Math");
    math();

    println!("\n");
    println!("bools");
    bools();

    println!("\n");
    println!("chars");
    chars();

    println!("\n");
    println!("tuples");
    tuples();


}


fn integers() {
    let i: u8 = 120;
    println!("{i} (u8)");

    let i: i8 = -120;
    println!("{i} (i8)")
}


fn math() {
    let sum: i32 = 200 + 31;
    println!("sum: {sum}");

    let diff: i32 = 20 - 38;
    println!("difference: {diff}");

    let prod: i32 = 20 * -3;
    println!("product: {prod}");

    // Division varies with ints and floats
    let quotient: f64 = 6.4 / 2.0;
    let trunc: u32 = 5 / 3; // equivalent to `//` operator in python

    println!("Float div (6.4 / 2) = {quotient}");
    println!("Integer div (5 / 2) = {trunc}");

    // remainder / modulo
    let rem: u16 = 40 % 6;
    println!("40 % 6 = {rem}");
}


fn bools() {
    let t: bool = true;
    let f: bool = false;

    if t == true {
        println!("t is true");
        
        }
    if f != true {
        println!("f is false")
    }
}


fn chars() {
    let c: char = 'c';
    println!("{c}")
}

fn tuples() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let t: (i32, i32, i32) = (100, 200, 300);
    println!("tuple = {:?}", x);
    println!("tuple.0 = {}", t.0);
    println!("tuple[0] does not work");
}
