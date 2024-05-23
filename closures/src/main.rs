// add num into scope
use num::Num;
use std::cmp::Reverse;

#[derive(Debug)]
struct Rectangle <T>
where
    T: Num
{
    width: T,
    length: T
}

impl <T> Rectangle <T>
where
    T: Num
{
    fn new(length:T, width:T) -> Rectangle<T> {Rectangle{length, width}}
}

fn main() {
    // Set up a closure to play around with
    let add_one = |x: i32| x + 1;

    // Test if a closure can be used multiple times
    let three = add_one(2);
    let four = add_one(3);

    println!("Three = {three} -- Four = {four}");
    // Yup, no ownership issues; it just returns a variable that is owned and the closure can be re-used

    // Closure types can also be inferred depending on the inputs
    let add_two = |x| x + 2;
    let _five = add_two(3); // the compiler inferred the type by using our input type here
    //let test = add_two("test".to_string()); <- fails because we are know forcing it infer using 2 diff types


    let mut list = vec![0,1,2,3];
    println!("Before closure: {:?}", list);

    // Create the test closure
    let mut mutable_borrow = || list.push(4); //<- list is immediately mutable borrowed

    //println!("{:?}", list); <- fails because an mutable borrow happens above and this is immutable

    // Call the closure; called like a func (it is an anonymous func after all)
    mutable_borrow();
    println!("After closure: {:?}", list);
    
    // Play with sorting objects using a key
    let mut rect_list = vec![
        Rectangle::new(8, 9),
        Rectangle::new(3, 4), // the values will be defined by the first definition; vector must contain same types
        Rectangle::new(10, 12)
    ];

    println!("Unsorted: {:?}", rect_list);

    // Now let's sort the rectangles by a key; width here
    rect_list.sort_unstable_by_key(|rect| rect.width);
    println!("Sorted: {:?}", rect_list);

    // Now reverse-order by width
    rect_list.sort_unstable_by_key(|r| Reverse(r.width)); // pretty basic use of Reverse built-in
    println!("Rev-sort: {:?}", rect_list);

}
