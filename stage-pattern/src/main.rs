use stage_pattern::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("I ate a saled for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a saled for lunch today", post.content());
}
