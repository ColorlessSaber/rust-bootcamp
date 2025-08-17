fn main() {
    // Two ways to initialize a vector
    // * using Vec::new(). Will have to use .push to put values into the vector.
    let v1:Vec<String> = Vec::new();
    let mut v1 = Vec::new();
    v1.push(String::from("One"));
    v1.push(String::from("Two"));
    v1.push(String::from("Three"));
    // * using vec!. Can define values to the vector at the start
    let v2 = vec![1,2,3];

    // how to index into a vector
    // * using square brackets
    let s = &v1[0]; // **NOTE** Cannot move values out of vectors. Have to use barrow.
    //let s1 = v.remove(0); // Removes the element from the vector and move the other elements over

    // * using get method. Safer method for it will not panic. If the index is valid, will get
    // the Option some variant; if the index is not valid, will get the Option none variant.
    let s2 = v1.get(0);

    if let Some(e) = s2 {
        println!("{}",e);
    }

    // iterate elements of a vector
    // * by borrowing the vector elements, you will not consume it.
    for s in &mut v1 {
        s.push_str("!");
    }

    for s in &v1 {
        println!("{}",s);
    }

    // * if all vector elements are consumed, the vector will be gone
    let mut v3 = vec![];
    for i in v1 {
        v3.push(i);
    }
}
