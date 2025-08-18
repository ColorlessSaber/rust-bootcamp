pub fn login(creds: models::Credentials) {
    // authenticate...
    crate::database::get_user();
}

fn logout() { // this function is private. ie, not available outside the module.
    // log user out...
}

pub mod models; // this is referencing the file called models.rs inside the folder auth_utils