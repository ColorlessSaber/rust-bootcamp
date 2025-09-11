use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_string(), 2);
    scores.insert("blue team".to_string(), 8);
    scores.insert("green team".to_string(), 6);

    let mut scores_iter = scores.iter(); // .iter() gives you an iterator over immutable references to items in the collection.
    //let mut scores_iter = scores.iter_mut(); // .iter_mut() gives you an iterator over mutable references to items in the collection.
    //let mut scores_iter = scores.into_iter(); // .into_iter() returns an iterator over the owned values in a collection.
    let first = scores_iter.next();

    // Rust will automatically refer which iterator to use in a for loop based on how the collection is past in.
    for (team, score) in &scores {
        println!("{team} Got: {score} points");
    }
}