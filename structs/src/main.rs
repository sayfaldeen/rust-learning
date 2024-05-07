fn _sec(title:&str){
    println!("\n{title}");
    println!("{}", "=".repeat(title.len()))
}

fn main() {

    _sec("Inrto to basic structs");
    // Set up a user object using the `User` struct
    //let user1 = User{
        //username : String::from("Sayf Al-Deen Hassouneh"),
        //email : String::from("shassouneh15@gmail.com"),
        //login_count : 0,
        //is_active : true
    //};

    //println!("{:?}", user1);

    // Make the struct mutable
    let mut user2 = build_user(
        String::from("Test User"),
        String::from("test.email@gmail.com")
        );

    // Print out before the change
    println!("{:?}", user2);

    // Change login_count for `user2`; simulate an additional login
    user2.login_count += 1;
    user2.is_active = true;
    //println!("{:?}", user2);

    // Create a third user but use the same email and different username; user2 as base (struct update syntax)
    let user3 = User {
        username: String::from("Test user 2"),
        ..user2
    };

    //println!("{:?}", user2); // => Error because of partial borrow ocurring when user3 is made
    println!("{:#?}", user3);

    // Make a color and a point
    //let black_color = Color(0,0,0);
    //let origin_point = Point(0.0,0.0,0.0);

    _sec("Advanced structs with implementations");
    let rect1 = Rectangle{width:10, height:20};
    //dbg!(&rect1); // this actually prints-out similar to {:#?} but also states which file and the line of the debug
                    // issue here is that `dbg!` also takes ownership so either borrow with a reference 
                    // or use it at the end
    println!("{:#?}", rect1);
    println!("The area for rect1 is {} and the perimeter is {}", rect1.area(), rect1.perimeter());

    let mut rect2 = Rectangle{width:5, height:10};
    rect2.set_width(100); // need to set rect2 as mutable if we want to do this
    rect2.set_height(99);
    //rect2.set_length(10); // does not work because this field does not exist
    println!(
        "{:?} -- {:?} -- can rect1 fit in rect2: {:?}", 
        rect1, rect2, rect1.can_fit_inside(&rect2)
        );

    let rect3 = Rectangle{width:21, height:11};
    println!(
        "{:?} -- {:?} -- can rect1 fit in rect3: {:?}", 
        rect1, rect3, rect1.can_fit_inside(&rect3)
        );

}

// Define a user struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    login_count: u32,
    is_active: bool
}

fn build_user(
    username:String,
    email: String,
    ) -> User {
    
    // Return User struct; use `Field-init shorthand`
    User {
        username,
        email,
        login_count: 0,
        is_active: true
    }
}

// Define structs without named fields to create different types
// The structure here uses tuples rather than curly brackets
struct Color (i32, i32, i32); // struct for a color
struct Point (f64, f64, f64); // struct for a 3-D point

//#################### Create a struct with data and methods ####################//
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
} // End struct Rectangle

// Add methods for Rectangle
impl Rectangle {

    // Define methods to mainpulate the rectangle parameters
    fn set_width(&mut self, width:u32) {
        self.width = width
    }

    fn set_height(&mut self, height:u32) {
        self.height = height
    }

    //fn set_length(&mut self, length:u32) {
        //self.length = length
    //} // Does not work because this field is not in the struct

    
    // Define method to calculate the area
    fn area(&self) -> u32 {
        self.width * self.height
    } // end area()


    // Define method to calculate the perimeter
    fn perimeter(&self) -> u32 {
        (2 * self.width) + (2 * self.height)
    } // end perimeter()


    // Define method to check if rectangle fits in another rectangle
    fn can_fit_inside(&self, other:&Rectangle) -> bool {
        let mut sorted_dims_self = vec![self.width, self.height];
        sorted_dims_self.sort();

        let mut sorted_dims_other = vec![other.width, other.height];
        sorted_dims_other.sort();

        sorted_dims_self[0] < sorted_dims_other[0] && sorted_dims_self[1] < sorted_dims_other[1]
    }

} // End impl Rectangle
