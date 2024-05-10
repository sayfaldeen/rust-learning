/*

   - This exercise code will work to expand on previous sections where we worked with module trees
   - The code for the different collections will each be stored in a different file
     - These files will then be called into scope and used to execute the code

*/

mod modules;

fn main() {
    let empty_vec = modules::vectors::make_new_empty_vector();
    let mut vec1 = empty_vec;
    vec1.push(0);
    vec1.push(1);
    vec1.push(2);

    println!("{:?}", vec1);

    let mut vec2 = modules::vectors::make_non_empty_vector();
    println!("{:?}", vec2);

    // Access the elements in the vector
    let third = vec2[2];
    // let third = &vec2[2]

    vec2.push(99);
    println!("{third}"); // this works because `third` is not defined using a reference

    // Handling out-of-bound index errors in vectors
    let index_100 = match vec2.get(100) {
            Some(val) => Some(val),
            None => {println!("No value was found"); None}
    };

    println!("index_100 = {:?}", index_100);

    // We can mutate a vector in place if we would like
    for i in &mut vec2 {
        *i += 100;
    }

    // We can also iterate through the vector using a for-in loop
    //let mut n = 0;
    //for i in &vec2 { // if we don't reference here, the ownership will be moved and we can't print at L45
        //println!("{n}) {i}");
        //n+=1
    //}

    modules::vectors::print_vec_contents(&vec2);

    println!("{:?}", vec2); // used to be L45; might move so I commented it here

    // You can conditionally return values from a vector as well
    enum DataCellType {
        IntData(i32),
        FloatData(f64),
        StringData(String)
    }
    
    // Return a value only if it is DataCellType::IntData
    let row = vec![
        DataCellType::IntData(10),
        DataCellType::FloatData(10.1),
        DataCellType::StringData("blue".to_string())
    ];

    // Iterate through the row vector and look at the types
    let mut n = 0;
    while n < row.len() {
        match row[n] {
            DataCellType::IntData(int) => println!("data at col {n} is an integer: {int}"),
            _ => println!("Data at col {n} is not an integer")
        };
        n+=1;
    }
}
