static mut COUNTER: u32 = 0;
/*
Accessing an immutable static variable is safe; however, accessing or modifying a mutable static
variable is unsafe.
 */

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    for _ in 0..10 {
        increment_counter();
    }

    unsafe {
        //println!("COUNTER: {}", COUNTER);
    }
}
