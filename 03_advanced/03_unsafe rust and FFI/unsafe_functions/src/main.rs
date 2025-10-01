fn main() {
    unsafe {
        my_function();
    }
}

// To make the function unsafe, place the "unsafe" keyword before it.
unsafe fn my_function() {
    println!("calling my function!");
}
