// blog_post.rs

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    // Create a new draft post
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    // Get the content of a published post
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // Add text to the draft post
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Request a review, transitioning to PendingReview state
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    // Approve the post, transitioning to Published state
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    // Create a new draft post
    let mut post = Post::new();

    // Add some text to the post
    post.add_text("I ate a salad for lunch today");

    // Request a review, transitioning to PendingReview state
    let post = post.request_review();

    // Approve the post, transitioning to Published state
    let post = post.approve();

    // Assert that the content is correct
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("Post content: {}", post.content());
}
