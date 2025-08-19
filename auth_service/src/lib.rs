#![allow(dead_code, unused_variables)]
// above disables warnings about dead code and unused variables

mod database; // this is referencing the file called database.rs
mod auth_utils; // this is referencing the file mod.rs inside folder auth_utils
// Rust preforms the following when searching for modules.
// * first look for a file with the same name as the module.
// * If it doesn't exist, it will then look for a folder with the same name as the module with a
// mod.rs file in it.
// **NOTE** the second isn't good coding for you can have multiple mod.rs files. Thus, it's better
// to create a folder with the name of the module to hold submodules in that module in question.

// The use statements are to bring objects and or functions in question into scope.
pub use auth_utils::models::Credentials; // the pub helps re-exporting the Credentials
use database::Status;
use rand::{rng, Rng};

pub fn authenticate (creds: Credentials) {
    let timeout = rng().random_range(100..500);
    println!("The timeout is {}", timeout);
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}