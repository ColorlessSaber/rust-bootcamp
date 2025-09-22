use std::{thread, time::Duration};

fn main() {
    // Thread spawn takes a closure containing the code, which will be run in the new thread.
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned Thread: {}", i);
            thread::sleep(Duration::from_millis(1)); // pauses the thread for one millisecond.
        }
    });
    for i in 0..10 {
        println!("Main Thread: {}", i);
        thread::sleep(Duration::from_millis(1)); // pauses the thread for one millisecond.
    }
    handle.join().unwrap(); // The handle is needed to make sure the main thread waits until the spawn thread is complete before terminating
}
