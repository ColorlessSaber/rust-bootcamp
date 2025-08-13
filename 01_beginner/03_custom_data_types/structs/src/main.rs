struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

fn main() {
    let mut book = Product {
        name: String::from("book"),
        price: 28.85,
        in_stock: true,
    };
    let price = book.price;
    book.in_stock = false;
    let sales_tax = calculate_sales_tax(&book);
    println!("Sales Tax: {}", sales_tax);
}


fn calculate_sales_tax(product: &Product) -> f32{
    product.price * 0.1
}