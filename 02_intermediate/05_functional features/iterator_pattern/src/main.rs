trait Iterator { // Rust's built in Iterator trait. It's implemented on a type which you can iterator over
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait IntoIterator { // Rust's built-in trait that is implemented on a type which you can turn into an iterator.
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

struct MyStruct {}

impl Iterator for MyStruct {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let mut m = MyStruct {};
    let item= m.next();
}
