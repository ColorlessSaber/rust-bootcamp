enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32,i32),
    Replace {
        from: String,
        to: String
    }
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from("{\"cmd\": \"undo\"}"),
            Command::Redo => String::from("{\"cmd\": \"redo\"}"),
            Command::AddText(text) => {
                format!("{{\
                \"cmd\": \"add_text\", \
                \"text\": \"{text}\" \
                }}")
            },
            Command::MoveCursor(line, column) => {
                format!("{{\
                \"cmd\": \"move_cursor\", \
                \"line\": {line} \
                \"column\": {column} \
                }}")
            },
            Command::Replace{from, to} => {
                format!("{{\
                \"cmd\": \"move_cursor\", \
                \"from\": \"{from}\" \
                \"to\": \"{to}\" \
                }}")
            }
        };
        json_string
    }
}

fn main() {
    // Basic example of using match
    let age = 35;
    match age { // must exhaust all possible matches, if not you will get an error
        1 => println!("Happy 1st Birthday1"),
        13..19 => println!("You are a teenager"),
        x => println!("You are {x} years old.") // x is a catch-all pattern
    }

    // Using match with an enum
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("Hello, World!"));
    let cmd3 = Command::MoveCursor(22,0);
    let cmd4 = Command::Replace{
        from: String::from("test"),
        to: String::from("sonic")
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
}
