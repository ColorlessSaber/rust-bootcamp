fn main() {
    let mut s1: String = String::from("Rust"); // heap allocated string

    // * ownership involving functions
    // print_string(s1); // the ownership of s1 is moved to the function print_string's p1
    print_string(s1.clone()); // create a new allocated string for the function print_string's p1
    let s_fn = generate_string(); // the ownership of the string generated inside generate_string is moved to s_fn
    println!("s_fn is {}", s_fn);
    let s_fn_ext = add_to_string(s_fn); // move ownership into add_to_string's p1 and then pass it back to s_fn_ext
    println!("s_fn_ext is {}", s_fn_ext);

    // * ownership involving variables
    let s3 = s1; // move ownership to s3
    let s4 = s3.clone(); // s4 and s3 now have their own allocated string in the heap
    println!("s3 is {}", s3);
    s1 = s3; // move ownership back to s1
    println!("s1 is {}", s1);

    // * ownership involving scope boundaries
    {
        let s2: String = String::from("hello");
    } // s2 is dropped here
    //println!("s2 is {}", s2); // won't work because s2 is inside the inner scope

    // for primitives that are stored on the stack--int, floats, boolean, char--are cloned by default
    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is {}", x);

} // s1 will be dropped

fn print_integer(i: i32) {
    println!("i is {}", i);
}

fn print_string(p1: String) {
    println!("p1 is {}", p1);
} // p1 is dropped

fn generate_string() -> String {
    String::from("Ferris")
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome");
    p1
}