use std::{io, fs, error, num::ParseIntError};

fn main() {
    let i = parse_file("example.txt");
    match i { // this is used with enum that handles the different error types
        Ok(i) => println!("{:#?}", i),
        Err(e) => {
            match e {
                ParseFileError::File => {
                    // ...
                },
                ParseFileError::Parse(e) => {
                    // ...
                }
            }
        }
    }
}
/*
fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> { // The error::Error is useful when you don't care what error type is returned
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}
*/

// Another way to handle different error types is to use an enum
// The advantage of this is the users who call the function can identify what error is being returned
enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}