struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        // To call a closure stored as a field in the struct, we need to wrap it in parentheses.
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    /*
    let validator = |username: &str, password: &str| -> bool { // can omit the return type for Rust can refer what data type is returned.
        !username.is_empty() && !password.is_empty()
    }; // the curly brackets are also optional if there is only one expression.
    */

    let weak_password = "password123!".to_string();
    // Fn - Immutably borrow variables in environment.
    // FnMut - Mutably borrow variables in environment. Can change environment.
    // FnOnce - Take ownership of variables in environment. The closure can only be called once.

    let validator2 = /*move*/|username: &str, password: &str| { // can use move to force ownership to the closure.
        !username.is_empty() &&
            !password.is_empty() &&
            password.len() > 8 &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) && // checks to see if the password has at lest one of the special characters
            password != weak_password // the closure immutable borrows weak_password value, given we define it with Fn in struct.
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2,
    };

    println!("{}", creds.is_valid());

    let password_validator = get_password_validator(8, true);
    let default_creds = get_default_creds(password_validator); // this compiles for the Box smart pointer can implement Fn, FnMut, and FnOnce

}

fn get_default_creds<T> (f: T) -> Credentials<T> where T: Fn(&str, &str) -> bool {
    Credentials {
        username: "guest".to_owned(),
        password: "password123!".to_owned(),
        validator: f,
    }
}

/*
// this works when return only one closure type
fn get_password_validator(min_len: usize, speical_char: bool) -> impl Fn(&str, &str) -> bool {
    move |_: &str, password: &str| !password.len() >= min_len // move is used to make the closure take ownership of min_len
}
*/

// When dealing with different possible closure types, need to use a smart box pointer.
fn get_password_validator(min_len: usize, speical_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if speical_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() >= min_len &&
                password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len) // move is used to make the closure take ownership of min_len
    }
}

/* Old way to validate the credentials
fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
 */