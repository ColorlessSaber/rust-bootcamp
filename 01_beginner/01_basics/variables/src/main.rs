fn main() {
    // creation
    let a: i16 = 5; // the i## defines the bit-number for the int value

    // mutability
    let mut b: i32 = 5;
    b = 10;

    // shadowing
    let c = 10;
    let c = 20; // this will shadow over the previous c declaration.

    println!("c is: {c}");

    // scope
    let d = 30;

    { // in Rust, scope is defined by curly braces.
        let d = 40;
        println!("inner d is: {d}");
        println!("c in inner is {c}");
        // let e = 40;
    }
    // println!("e is: {e}");

    println!("d is: {d}");

}
