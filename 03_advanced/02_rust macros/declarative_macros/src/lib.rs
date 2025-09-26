/*
Declarative Macros -- Also known as "macros by example" are a way of writing macros by matching
against code similar to a match expression.
 */

/*
Macro format
macro_rules! name {
    rule0;
    rule1;
    ...
    ruleN;
}
ruleN -- (matcher) => { expansion aka transcriber }
 */
use std::collections::HashMap;

#[macro_export]
macro_rules! hello {
    () => (
        println!("Hello world!")
    );
}

#[macro_export]
macro_rules! map {
    /*
    Meta variables use the following format: $[identifier] : [fragment-specifier]
     */
    ($key:ty, $val:ty) => {
        {
            let map: HashMap<$key, $val> = HashMap::new();
            map
        }
    };
    /*
    Repetition syntax uses the following format: $(...) sep rep
    (...) -- the matcher to be repeated
    sep -- optional separator token
    rep --  required repeat operator

    for below, ',' is the separator and '*' is the repeat operator.
    '*' represents zero or more repetitions.
    '?' represents at most one repetition.
    '+' indicates one or more repetitions.
     */
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)* // insert one line of code for each repetitions.
            map
        }
    };
}