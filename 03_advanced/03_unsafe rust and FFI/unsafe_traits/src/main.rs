unsafe trait MyTrait {
    fn some_function(&self);
}
/*
A trait is marked unsafe to warn implementers that certain invariants must be respected or
memory safety will be violated.
**NOTE** Unsafe traits aren't seen that often.
 */


// when a trait is marked unsafe, the impl block needs to be marked unsafe as well
unsafe impl MyTrait for String {
    fn some_function(&self) {
        // ..
    }
}

fn main() {
    let s = "some string".to_owned();
    s.some_function();
}
