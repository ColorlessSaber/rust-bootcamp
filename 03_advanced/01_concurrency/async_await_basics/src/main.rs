#[tokio::main] // The crate "tokio" helps to allow us to define the main as async.
async fn main() {
    let f = my_function();
    println!("Let's Get Rusty");
    f.await;
}
/*
At the top most level, we need a runtime or an executor to drop the future(s) to completion.
The runtime is responsible for polling the top level futures and running them to completion. Its
also responsible for running multiple futures in parallel.
Given Rust does not have an async runtime available, we will need to use community built Crate
called "tokio."
 */

async fn my_function() {
    /*
    The .await keyword will attempt to run the future to completion. It also pauses execution of the current future
     yielding control back to the runtime.
     */
    println!("I'm an async function!");
    let s1 = read_from_database().await; // .await will attempt to run the future to completion.
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

async fn read_from_database() -> String {
    "DB Result".to_owned()
}

/*
Async await is special syntax which allows us to write functions, closures, and blocks
that can pause execution and yield control back to the runtime, allowing other code to make progress
and then pick back up from where they left off.
Also, Async is syntax sugar. It implements the trait Future and that traits implements the enum Poll.

trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
 */