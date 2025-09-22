use std::thread;
use std::sync::mpsc; // mpsc is used to allow multiple threads to send messages and a single thread to receive messages.

fn main() {
    let (tx, rx) = mpsc::channel();
    let sentences = [
        "!dlrow olleh".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR teG s'tel".to_owned(),
        "!tsuB ro tsuR".to_owned()
    ];

    for s in sentences {
        let tx_clone = tx.clone(); // make a "copy" of the transmitter
        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap(); // Send the value out of hte thread
        });
    }
    drop(tx); // need to do this so rx doesn't continue to listen for transmitted signals from threads.
    for sentence in rx {
        println!("{sentence}");
    }
}
