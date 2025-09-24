use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Database {
        Database { connections: vec![] }
    }
    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    let db = Arc::new(Mutex::new(Database::new()));

    let mut handles = vec![];

    for i in 0..20 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });// When db_lock drops here, it releases the lock; Mutex implements the drop trait.
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let db_lock = db.lock().unwrap();
    println!("{db_lock:?}");
}