use rand::{random_bool, rng, Rng};

fn main() {
    /* p1 and p2 have same lifetime
    let player1 = String::from("Player 1");
    let player2 = String::from("Player 2");

    let result = first_turn(player1.as_str(),player2.as_str());

    println!("Player going first is: {}", result);
     */

    let player1 = String::from("Player 1");
    let result: &str;
    {
        let player2 = String::from("Player 2");
        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result); // this works for result has the same lifetime as player1
}

/* p1 and p2 share generic lifetime annotation
fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rng().random::<bool>() {
        p1
    } else {
        p2
    }
}
*/

// output shares the same lifetime as p1
fn first_turn<'a>(p1: &'a str, p2: &str) -> &'a str {
    p1
}

// An example of a static lifetime--lasts the entre length of the program
fn static_first_turn(p1: &str, p2: &str) -> &'static str {
    let s: &'static str = "Let's get rusty!";
    s
}