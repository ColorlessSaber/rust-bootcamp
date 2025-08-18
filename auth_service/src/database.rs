pub enum Status { // The pub is short for public. It makes this enum public; ie, available outside the module and the crate it is.
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> Status {
    return Status::Connected;
}

pub fn get_user() {
    // get user from database...
}