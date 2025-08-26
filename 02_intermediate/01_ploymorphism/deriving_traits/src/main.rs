#[derive(Debug, PartialEq)] // This is a derived macro. it implements the traits in the parenthesis.
struct Point {
    x: i32,
    y: i32,
}
// **NOTE** Not all traits can be used with the derived macro. Only traits with associated derived trait
// can be used.

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
    println!("{:?}", p1 == p2);
    println!("{:?}", p1 == p3);
}