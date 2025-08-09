fn main() {

    // *** Scaler Data Types - store only one value ***
    // boolean
    let b1: bool = true;

    // unsigned integers - positive numbers only
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // signed integers - positive and negative numbers only
    let i6: i8 = 1;
    let i7: i16 = 1;
    let i9: i32 = 1;
    let i10: i64 = 1;
    let i11: i128 = 1;

    // floating point numbers
    let f1: f32 = 1.1;
    let f2: f64 = 1.2;

    // platform specific integers - not used often
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str, and String
    // Note: s1 and s2 are different
    let c1: char = 'a'; // single character
    let s1: &str = "hello"; // string slice
    let s2: String = String::from("Hello");

    // *** Compound Data Types - store multiple values ***
    // arrays - holds multiple values of the same type
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let i_a1 = a1[4]; // how to index an array

    // tuples - holds multiple values of different types.
    let t1 = (1,2,3);
    let t2 = (5, 5.0, "5");

    let s1 = t2.2; // how to index a tuple
    let (i1, f1, s1) = t2; // destructure a tuple

    let unit = (); // an empty tuple. These are returned when no other meaningful value can be returned.

    // Type aliasing
    type Age = u8;
    let a1: Age = 57;

    let number_of_friends: u32;
    number_of_friends = 10;
    println!("{}", number_of_friends);

}
