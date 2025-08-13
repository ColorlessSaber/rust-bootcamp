struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    // associated functions -- they don't take self as parameters; they aren't called via dot method
    fn new(name: String, price: f32) -> Product {
        // this associated function is useful for you can do calculations and set default fields
        // before building the struct
        Product {
            name,
            price,
            in_stock: false
        }
    }
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // methods
    fn calculate_sales_tax(&self) -> f32{ // use &self if you wish to borrow the struct; ie, not drop it.
        self.price * Product::get_default_sales_tax()
    }
    fn set_price(&mut self, price: f32) { // use &mut self to change an field
        self.price = price;
    }
    fn buy(self) -> i32 { // use "self" when you want the struct to be dropped after the function is ran
        let name = self.name;
        println!("{} was bought!", name);
        123 // receipt number
    }
}

fn main() {
    let mut book = Product::new(String::from("book"), 30.0);
    let price = book.price;
    book.in_stock = false;
    let sales_tax = book.calculate_sales_tax();
    println!("Sales Tax: {}", sales_tax);
    book.set_price(1.0);
    book.buy(); // struct book is dropped
    //book.set_price(28.5); // invalid for the struct has been dropped via .buy()
}