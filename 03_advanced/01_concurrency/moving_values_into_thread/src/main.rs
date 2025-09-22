use std::thread;

fn main() {
    let s = "Let's Get Rusty".to_owned();

    let handle = thread::spawn(move || { // Use the "move" keyword to move a value into the spawn thread.
        println!("{s}");
    });
}
