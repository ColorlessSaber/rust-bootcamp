use blog_shared::Post;

fn main() {
    let post = Post::new(
        "post to the server".to_owned(),
        "Let's get rusty!".to_owned(),
    );

    println!("{:#?}", post);
}
