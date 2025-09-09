struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonalIterator {
    values: Vec<String>,
}

impl Iterator for PersonalIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.remove(0))
    }
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonalIterator;
    fn into_iter(self) -> Self::IntoIter {
        // **NOTE** not necessary to do this given a vector can be convertor into an iterator using .into_iter() method.
        PersonalIterator {
            values: vec! [
                self.last_name,
                self.first_name,
                self.occupation
            ],
        }
    }
}

fn main() {
    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };
    /* Using the next method
    let mut i = p.into_iter();
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    */

    for item in p { // The for-loop knows how to handle iterators. When None is return, the loop exits
        println!("{}", item);
    }

}
