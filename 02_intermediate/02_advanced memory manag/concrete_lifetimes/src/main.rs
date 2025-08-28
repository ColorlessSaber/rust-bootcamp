fn main() {
    /* Example of Ownership
    let s1 = String::from("Let's get rusty!");
    println!("s1: {s1}");
    let s2 = s1; // lifetime of s1 ends here
    println!("s1: {s1}");
     */

    /* Example of Borrowing
    let r1: &String;
    {
        let s1 = String::from("Let's get rusty");
        r1 = &s1;
        println!("r1: {}", r1);
    }

    // println!("r1: {}", r1); // The barrow of s1 does not live long enough for r1 to be printed
     */

    let mut s1 = String::from("Let's get rusty");
    let r1 = &s1;
    println!("r1: {}",r1); // lifetime of r1 ends here
    let r2 = &mut s1;
    r2.push_str("!"); // lifetime of r2 ends here

}
