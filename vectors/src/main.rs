fn main() {

    // Create a new empty vector and push to it
    let mut v: Vec<i64> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);

    println!("v: {:?}", v);

    // Create a vector using the macro
    let mut v = vec![0.0, 1.0, 2.0];
    println!("v: {:?}", v);

    // Check for an index in a vector 
    let v0 = v.get(0).expect("Index out of bounds"); 
    println!("v[0] = {:?}", v0);

    //let v4 = v.get(4).expect("Index out of bounds"); <- panicks due to OOB index
    //println!("v[4] = {:?}", v4);

    // Iterate through an array and modify in place
    for i in &mut v {
       *i += 100.0; // need to de-reference because operations need access to the actual
                        // value and not just the reference
    }
    println!("vec +100: {:?}", v);

}
