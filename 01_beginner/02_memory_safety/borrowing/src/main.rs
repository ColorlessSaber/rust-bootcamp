fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1; // creates a reference to s1; IE, borrowing the value form s1.
    prt_string(r1);
    let r2 = &mut s1; // this line of code has to be here to obey the borrowing rules.
    add_to_string(r2);
    println!("s1 is {}", s1);

    let s2 = generate_string();
    println!("{}", s2);

    // Example of creating a string_pointer that can change what it is pointing to
    let mut str1 = String::from("modifiable");
    let str2 = String::from("fixed string");
    let mut str_ptr: &String;
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");
    str_ptr = &str2;
    println!("ptr currently points to {str_ptr}");
    str1.push_str(" string");
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");

}

fn generate_string() -> String {
    String::from("Ferris")
}

/*
// Example of a dangling reference
fn generate_string() -> &String {
    let s = String::from("Ferris");
    &s // this would lead to a dangling reference
} // s will be dropped
 */


fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome"); // Rust automatically de-references p1
}

fn prt_string(p1: &String) { // need to define that the input value is a reference string
    println!("p1 is {}", p1);
}
