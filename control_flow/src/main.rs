use std::array;

fn main() {

    _section("control flow");
    let val = 4;
    let x = less_than_3(val);
    println!("4 < 3 = {x}");

    if x == true {
        println!("condition is true");
    }
    else {
        println!("condition is false");
    }

    // We can also assign values using control flow
    let size = if x {"smaller"} else {"larger"};
    println!("{val} is {size} than 3");


    let val = 1;
    let x = less_than_3(val);
    println!("1 < 3 = {x}");

    if x == true {
        println!("condition is true");
    }
    else {
        println!("condition is false");
    }

    // We can also assign values using control flow
    let size = if x {"smaller"} else {"larger"};
    println!("{val} is {size} than 3");

    // Test out looping
    _section("loops");
    loop_loops(15);

    // Test out nested and named loops
    _section("nested loops");
    loop_nested();

    // Try while loops
    _section("while loops");
    while_loops(10);

    // Try for loops
    _section("for loops");
    let list = vec!["a", "b", "c"];
    str_for_loops(&list);
}

fn _section(name:&str) {
    println!("\n");
    println!("{name}");
    println!("{}", "=".repeat(name.len()))


}

fn less_than_3(x: i32) -> bool {
    x < 3
}

fn loop_loops(limit: i32) {
    let mut count = 0;

    let res = loop {
        count += 1;

        if count == limit {
            break count * 2
        }
    };

    println!("Limit: {limit} -- Results: {res}")

}

fn loop_nested() {
    // init the count
    let mut count = 0;
    println!("count = {count}");

    // start outer loop
    let mut rem = 10;

    'outer_loop: loop {
        println!("remaining: {rem}");

        // start inner loop
        'inner_loop: loop {
            if count == 2 {
                break 'outer_loop;
            }

            if rem == 9 {
                break 'inner_loop;
            }
            rem -= 1
            

        } // end inner_loop
        
        count += 1

    } // end outer_loop
    println!("End count: {count}")
}

fn while_loops(count:u32) {
    let mut count = count;

    while count != 0 {
        println!("T-minus: {count}!");

        count -= 1;

    }
    println!("LIFTOFF!")
}

fn str_for_loops(list: &[&str]) {
    let mut _n = 0;
    for i in list{
        println!("{_n}: {i}");
        _n += 1
    }
}
