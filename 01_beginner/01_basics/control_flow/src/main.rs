fn main() {
    // if/else
    let a = 5;

    if a > 5 { // the conditions in if statements just be boolean.
        println!("a is bigger than 5");
    } else if a > 3 {
        println!("a is bigger than 3");
    } else {
        println!("a smaller or equal to 3");
    }

    let b = if a > 5 { 1 } else { -1 };

    // loop
    loop {
        println!("loop forever!");
        break; // this will break the loop
    }

    // ** for loops, you can label the outer loop so you can break the outer loop from an inner loop.
    // otherwise, the break will only break the inner-loop and not the outer
    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("inner loop");
            break 'outer;
        }
    }

    // ** can set a let-statement to a loop. Not really useful in most cases.
    let x = loop {
        break 5;
    };

    // While loops
    let mut counter: i32 = 0;

    while counter < 5 {
        println!("a is {}", counter);
        counter += 1;
    }

    // for loops
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
        println!("The value is {}", element);
    }

}
