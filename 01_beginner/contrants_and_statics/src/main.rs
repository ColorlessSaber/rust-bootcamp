const MAX_PLAYERS: u8 = 10; // A constant variable cannot be made mutable
static CASINO_NAME: &str = "Rusty Casino"; // A static variable can be made mutable

fn main() {
    // The value of the constant will be inlined
    let a = MAX_PLAYERS;
    let b = MAX_PLAYERS;

    // static variables accupy a specific place in memory, thus there can only be one instance of it.
    let c = CASINO_NAME;
    let d = CASINO_NAME;
}
