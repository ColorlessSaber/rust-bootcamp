fn main() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("Username: {}", name);
    }
}

fn get_username(user_id: i32) -> Option<String> {
    let query = format!("Get username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    db_result.ok() // converts the Result enum to Option Enum (Ok() -> Some())
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty(){
        Err(String::from("Query string is empty!"))
    } else{
        Ok(String::from("Ferris"))
    }
}

/* The structure of the Result enum.
enum Result<T, E> {
    Ok(T),
    Err(E),
}
 */