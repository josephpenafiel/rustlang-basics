#![allow(dead_code, unused_variables)]

use blog::Post;
mod peripherals;

fn main() {
    let mut post = Post::new();

    post.add_text("this is a blog Post");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("this is a blog Post", post.content());
    println!("{}", post.content());

    peripherals::test();
    peripherals::gpioa::test();
    peripherals::gpioa::moder::test();
}

mod blog {

    pub struct Post {
        //state: Option<Box<dyn State>>,
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    struct Draft;

    struct PendingReview;

    struct Published;

    //defines the behavior shared by different post states
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview)
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    impl State for PendingReview {
        // returns itself because when a review is requested on PendingReview it should
        // hold as pending review
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published)
        }
    }

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        } // end new

        pub fn content(&self) -> &str {
            &self.content
        } // end content
    } // end impl

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        } // end request_review
    } // end impl

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}
