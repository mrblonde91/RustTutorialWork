pub struct Post {
    state: Option<Box<dyn State>>.
    content: String,,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            ""
        }

        pub fn request_review(&mut self){
            if let Some(s) = self.state.take(){
                self.state = Some(s.request_review())
            }
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})   
    }
}

struct PendingReview {}

impl State for PendingReview {
    //invalidates old state and takes it
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}