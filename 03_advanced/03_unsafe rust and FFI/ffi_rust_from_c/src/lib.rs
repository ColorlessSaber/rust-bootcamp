#[unsafe(no_mangle)] // this will prevent the compiler from changing the name of the function during compilation.
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*
Once everything is set up, we run cargo build.
Next is we link the Rust library to C.
''
clang src/call_rust.c -o call_rust -lffi_rust_from_c -L./target/debug
''
 */