use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer {value}
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut s = MySmartPointer::new(Box::new("Let's get Rusty".to_owned()));
    // let s = &(***s); // de-reference MySmartPointer to Box Smart Pointer.
    print(&mut s); // This compiles due to de-ref coercion. &Box -> &String -> &str
    // Rust coerces one type to another type, happens when passing a value into a function or method.
    // This only works for types that implement the Deref or DerefMut
    // A mutable reference can be coerced to an immutable reference, but the opposite is not true.
}

fn print(s: &str) {
    println!("{}", s);
}