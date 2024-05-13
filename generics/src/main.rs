/*
   - define a function to find the largest numbers
   - This function needs to be able to take in integers and floats
*/

fn find_largest<T: PartialOrd + Copy>(v: Vec<T>) -> T {
    let mut largest = v[0]; // instantiate with first value
    for val in v {
        if val > largest {largest = val}
    }
    largest
}

// Create a struct that takes different types; second type can be different than first
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

// Set up implication block to work with all data types
impl <T, U> Point<T, U> {

    fn x(&self) -> &T {&self.x}

    fn mixup<V,W> (&self, other:Point<V, W>) -> Point<&T, W> {
        Point{x: &self.x, y: other.y}
    }
}

// Set up impl block that works only on f64 data
impl <T> Point<T, f64> {
    fn y(&self) -> f64 {self.y}
}

fn main() {

    // Create a list of numbers
    let num_list = vec![1,2,3,4,5];
    let largest = find_largest(num_list);
    println!("The largest number is {}", largest);


    // Explore the struct that takes in different data types
    let point1 = Point{x:1, y:2.1};
    println!("{:?}", point1);
    println!("point1 xy = {}, {}", point1.x(), point1.y());

    let point2 = Point{x:0, y:0};
    //println!("{}", point2.x(), point2.y()) <- fails because y is i32, not f64
    println!("{:?}", point2);

    let point3 = point1.mixup(point2);
    println!("{:?}", point3);
    //println!("p1: {:?} -- p2: {:?}", point1, point2); <- fails because we are borrowing a moved value (point2)

    /*
       - This occurs because mixup() is not using a reference for other
       - This can be fixed by taking a reference to other but also indicating the lifetime
        - Have not gotten to this lesson yet, so I will not implement lifetimes here and muddle things up
    */

}
