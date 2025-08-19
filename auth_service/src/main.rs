/*
The package "auth_service" is to understand package structure and module structure.
It's not designed to be used or implemented in any way.
 */

use auth_service::{Credentials, authenticate};
fn main() {
    let creds = Credentials {
        username: "letsGetRusty".to_string(),
        password: "password123".to_owned()
    };

    // auth_service::authenticate(creds); // another way to call from the lib.rs
    authenticate(creds);
}