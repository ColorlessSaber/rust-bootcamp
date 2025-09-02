use std::fs::File;

fn main() {
    /* One way to handle recoverable_error
    let file = File::open("example.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file: {:?}", error);
        }
    };
     */


    // **NOTE** Both .unwrap and .except are great when prototyping.
    // the .unwrap() will handle the Result-enum. If Ok is returned, it will pass it to file.
    // if Err is returned, .unwrap() will panic.
    let file = File::open("example.txt").unwrap();

    // .except() is similar to .unwrap(), only difference we can provide a custom failure message
    let file1 = File::open("hello.txt").expect("Failed to open hello.txt");

}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username cannot be empty".to_owned())
    } else {
        Ok(1)
    }
}