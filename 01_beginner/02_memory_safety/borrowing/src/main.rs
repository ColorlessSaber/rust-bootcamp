fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1; // creates a reference to s1; IE, borrowing the value form s1.
    prt_string(r1);
    let r2 = &mut s1; // this line of code has to be here to obey the borrowing rules.
    add_to_string(r2);
    println!("s1 is {}", s1);

    let s2 = generate_string();
    println!("{}", s2);

}

fn generate_string() -> String {
    String::from("Ferris")
}

/*
Example of a dangling reference
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
