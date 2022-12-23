fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("after add text: post.content: {}", post.content());

    let post = post.request_review();
    println!("after request review: post.content: {}", post.content());

    let post = post.approve();

    println!("after approve: post.content: {}", post.content());
}

pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn content(&self) -> &'static str {
        ""
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn content(&self) -> &'static str {
        ""
    }

    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
