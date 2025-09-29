use procedural_macros::*;
use attribute_macro::*;

// *** derive macro ***
trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Log)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Database {
        Database { url, connections: 0 }
    }

    fn connect(&mut self) {
        self.info(format!("new connection to {}", self.url).as_str());
        self.connections += 1;
        if self.connections >= 100 {
            self.warn(format!("100 or more connections open!").as_str());
        }
    }
}
// *** end of derive macro ***

// *** attribute like macro ***
#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

#[log_call(verbose = true)]
fn buy_product(laptop: Product, price: u32) {
    println!("buy product with {} at {}", price, laptop.name);
}

// *** end of attribute like macro ***


fn main() {
    // Example of functional macro
    log_info!([TIME] starting program...);

    // example of derived macros
    let mut db = Database::new(String::from("localhost:5433".to_owned()));
    for _ in 0..100 {
        db.connect();
    }

    // attribute like macro
    let laptop = Product { name: "MacBook Pro".to_owned(), price: 2000};
    buy_product(laptop, 20);
}