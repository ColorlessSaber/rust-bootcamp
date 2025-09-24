use tokio::time::sleep;
use std::time::Duration;

/*
Tokio crate uses a thread pool to allow multiple threads to run in parallel.
If you wish to force tokio to run threads in slices (IE, not run in parallel), use "flavor = "current_thread"
 */

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    handles.push(tokio::spawn(async {
        let _res = tokio::task::spawn_blocking(|| {
            expensive_computation()
        });
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(10)).await; // Tokio's sleep will stop the current future from executing for a given duration
    "DB Result".to_owned()
}

fn expensive_computation() {
    let mut i = 0;
    for _ in 0..400_000_000 {
        i += 1;
    }
    println!("Done with expensive computation! i = {i}");
}
