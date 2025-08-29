use std::rc::Rc;

struct Database {}

struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

fn main() {
    let db = Rc::new(Database {}); // The Rc smart pointer allow several instances to share "ownership."
    // **NOTE** Can only be used in single threaded applications.
    /*
    The Rc::clone does not clone the database instance. It will simply increment the reference count.
     */
    let auth_service = AuthService { db: Rc::clone(&db) }; // Rc count: 1
    let content_service = ContentService { db: Rc::clone(&db) }; // Rc Count: 2
} // Rc Count: 0
