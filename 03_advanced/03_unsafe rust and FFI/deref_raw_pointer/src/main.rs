fn main() {
    let mut s = "Let's Get Rusty".to_owned();

    // raw pointers created by casting
    let raw1 = &s as *const String; // immutable raw pointer -- denoted by *const followed by the data type they point to
    let raw2 = &mut s as *mut String; // mutable raw pointer -- denoted by *mut followed by the data type they point to
    /*
    The 'as' is used to casting the reference to a raw pointer.
     */

    // raw pointer pointing to invalid memory
    let address = 0x012345usize;
    let raw3 = address as *const String;

    /*
    It is safe to create raw pointers; it's not safe to deference raw pointers.
     */

    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is {}", *raw1);
    }
}
