struct BrowserCommand<T> {
    name: String,
    payload: T,
}

// need to define the generic types after impl and after struct_name
// Reason why we need to do this is so Rust knows we are implementing functionality for any type
// than a concrete type.
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }

}

// we can create another impl for the struct and define what the generic type is, so Rust knows
// how to treat the generic type
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("payload: {}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrust.com".to_owned()
    );

    let cmd2 = BrowserCommand::new (
        "zoom".to_owned(),
        200
    );
    cmd1.print_payload();
    // cmd2.print_payload(); // won't work because "payload" is int.

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}

fn serialize_payload<T>(payload: T) -> String {
    // concert payload to json string...
    "placeholder".to_owned()
}