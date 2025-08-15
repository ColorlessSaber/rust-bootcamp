fn main() {
    let username = get_username(1);
    /*
    //Can use match to handle option conditions
    match username {
        Some(name) => println!("Username: {}", name),
        None => println!("Username not found"),
    }
    */
    // Or can use below if you only care about Some() and not None
    if let Some(name) = username {
        println!("Username: {}", name);
    }
}

fn get_username(user_id: i32) -> Option<String> {
    // get username from database
    let db_result = String::from("Ferris");
    if user_id == 1{
        Some(db_result)
    } else {
        None
    }
}
