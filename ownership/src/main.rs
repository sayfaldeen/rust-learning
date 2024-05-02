// Code containing examples to work through and understand ownershipin Rust

fn main() {

    _sec("variable calling");
    call_variable();

    _sec("variable copying");
    variable_copying(32);

    _sec("box in heap");
    box_in_heap(1);
    
    _sec("collections using heap (vector)");
    let mut v = vec![10;5];
    collections_heap(&mut v);

    _sec("derefrencing pointers");
    let x = Box::new(1);
    derefrencing_pointers(x)
}


fn _sec(name:&str) {
    println!("\n");
    println!("{name}");
    println!("{}", "=".repeat(name.len()))
}

fn call_variable() {
    // Below block fails because x is not yet defined; this may cause undesired behavior if it were to be compiled
    //println!("{x}");
    let x = true;
    println!("Does this work? {x}")
}

fn variable_copying(var:i32) {
    // This function will explore variables in a manner that allows them to be copied
    let a = var; // stack 1
    let mut b = a; // stack 2
    b += 1; // stack 3

    println!("{var} + 1 = {b}");

    // With this code we end up with a 3 copies of the `a` variable
    // 1. containing original `a` in its own stack
    // 2. `a` = 5 and `b` = 5 in own stack
    // 3. `a` = 5 `b` = 6 in own stack

}

fn box_in_heap(var:i32) {
    let a = Box::new([var; 1_000_000]); // this stores the array on the heap
    let b = a; // this generates a pointer
    println!("{}", b.len());
}

fn collections_heap(vec:&mut Vec<i32>){
    println!("original vector: {vec:?}");
    //println!("{vec:#?}"); // long-format printing
    let _sum1: i32 = vec.iter().sum(); // the format of sum must be specified and MUST be the same as the type within the vector
    
    vec.push(20);
    println!("modified vector: {vec:?}");
    let _sum2: i32 = vec.iter().sum();

    println!("Vector1 sum = {_sum1}");
    println!("Vector2 sum = {_sum2}")

}

fn derefrencing_pointers(mut x: Box<i32>) {
    let a: i32 = *x;  // here we are reading the heap value provided but not referencing to it
    *x += 1;         // this enables us to modify the heap value

    let r1: &Box<i32> = &x;  // here we are pointing to the stack (x) which points to the heap
    let b:i32 = **r1;       // two dereferences gets the heap value but does not point to it (deref)
                           // because we are pointing to the stack
    *x += 2;
    let b2: i32 = *x;       // let's see what happens here

    let r2: &i32 = &*x;      // this points to the heap 
    let c: i32 = *r2;       // since r2 points to the heap, we only need one deref

    // Print out the results of all the variables
    println!("a = {a}");
    println!("b = {b}");
    println!("b2 = {b2}");
    println!("c = {c}");

    /*
       derefrencing pointers
       =====================
        a = 1
        b = 2
        b2 = 4
        c = 4
    */

}
