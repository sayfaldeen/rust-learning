/*

This module is meant to play around with enums a little more and add functionality to them

*/

#[derive(Debug)]
enum ProductType{
    Food,
    Clothing,
    Electronic,
    Book
}

impl ProductType {
    fn clearance_factor(&self) -> f64 {
        match self {
            ProductType::Food => 0.8, // 20% discount
            ProductType::Electronic => 0.75, // 25% discount
            _ => 0.65 // 35% discount
        }
    }
}

#[derive(Debug)]
struct Product {
    name: String,
    product_type: ProductType,
    base_price: f64,
    items_remaining: u32,
    clearance: bool
}

impl Product {
    fn price(&self) -> f64 {

        if self.clearance { self.base_price * self.product_type.clearance_factor() } // 40% sale for clearance items
        else { self.base_price }
    }

}

fn main() {
    // Construct a product
    let book = Product{
        name: "GoT".to_string(),
        product_type: ProductType::Book,
        base_price: 30.0,
        items_remaining: 20,
        clearance: true
    };

    println!("The price for {} is ${}", book.name, book.price());

    let apple = Product{
        name:"Granny Smith apple".to_string(),
        product_type:ProductType::Food,
        base_price: 9.99,
        items_remaining: 10,
        clearance: true
    };

    println!("The price for {} is ${}", apple.name, apple.price());
}
