fn main() {
    let ten = 10;
    let greater_than = |x: &i32| *x > ten;
    //let less_than = |x: &i32| *x < 20;

    let result = are_both_true(greater_than, less_than, &9);
    println!("{}", result);
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    // **NOTE** best to use closure defining syntax to allow function pointers and or closures to pass in without any errors.
    f1(item) && f2(item)
}