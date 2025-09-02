use std::{fs, io};

fn main() {
    let first_line = read_first_line("example.txt");
}
/* One way to convert Option to Result is to wrap the Ok value in an Option
fn read_first_line(filename: &str) -> Result<Option<String>, io::Error> {
    fs::read_to_string(filename).map(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}
 */

// Use a "combinator" (Ok, map, and_then, etc) -- A function that perform operations on a value
// and can even change the value.
// You can chain them.
fn read_first_line(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok().and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}