#[link(name="adder", kind="static")]
unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}
/*
The "extern" keyword is used to declare function interfaces of external code, which Rust can call.
The string "C" after extern defines which application binary interface (ABI) the external function uses;
it defines an interface between binary programs. We also need to place "unsafe" before extern.

**NOTE** Rust compiler cannot check that the types we give these arguments are correct.

The macro #[link] is used to tell Rust to link against our adder library.

We need to use the terminal command "clang" to turn adder into an object file.
``
clang -c adder/adder.c -o adder/adder.o
``

And then use the ar utility to turn the object file into a library.
``
ar -rc ./adder/libadder.a ./adder/adder.o
``

Lastly, we need to set the Rust flags environment variable to link against the library and then run
the Rust program.
``
RUSTFLAGS = "-L ./adder" cargo run
``
 */

fn main() {
    let x: i32;
    unsafe {
        x = add(1, 2);
    }
    println!("x is: {}", x);
}
