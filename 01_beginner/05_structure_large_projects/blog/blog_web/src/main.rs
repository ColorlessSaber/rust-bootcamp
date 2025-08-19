use blog_shared::Post;

fn main() {
    let post = Post::new(
        "post on the web".to_owned(),
        "lets get rusty".to_owned(),
    );

    println!("{:?}", post);
}
