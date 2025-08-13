fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs
    // * by using tuple structs, Rust will enforce that the correct datatype is passed into,
    // * correct number of elements are passed, and it makes it more clear what the variable
    // * is being passed around
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let color1: RGB = RGB(255, 106, 0);
    let color2 = CMYK(0, 58, 100, 0);

    // unit-like structs
    //* These aren't passed any values and are rarely used; mostly used for storing traits
    struct myStruct;
}
