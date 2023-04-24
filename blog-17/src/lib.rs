pub trait State {
    /*
        Box<Self> 只能被 Box 包裹的实例调用
        这里会获取 self 的所有权，并将旧值失效
     */
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

}
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post { state: Some(Box::new(Draft {})), content: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        /*
            返回值的引用 Option<&T>
            这里 unwrap 不会 panic 因为 T 总是有值
         */
        self.state.as_ref().unwrap().content(&self)
        
    }

    pub fn request_review(&mut self) {
        // 
        /*
            使用 take 函数取出 some 的值，同时所有权 move 出来了
            self.state.take() 之后 self.state 的值就是 None
         */
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}



struct Draft {

}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {

}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {

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