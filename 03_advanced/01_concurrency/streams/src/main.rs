use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    // Just like iterators, Steams are able to support combinations, like .maps.
    let mut stream = tokio_stream::iter(vec!["let's", "Get", "Rusty"])
        .map(|s| s.to_ascii_uppercase());

    /*
    As of 09/24/2025, Streams are not supported in for loops. Which is why we are using a while loop
    instead.
     */
    while let Some(item) = stream.next().await {
        println!("{}", item);
    }
}
