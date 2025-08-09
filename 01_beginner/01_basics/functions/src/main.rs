fn main() {
    let z = my_function(22);
    println!("The value of z is: {}", z);

    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));

}

fn my_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    let y = 10;
    y
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}