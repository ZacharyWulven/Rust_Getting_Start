pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    // new 时返回草稿 Post（DraftPost）
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 获得所有权，并返回新 struct
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    /*
        只有发布成功返回 Post，Post 才有 content 方法
        达到了未 approve 前不能查看其内容目的
     */
    // 获得所有权，并返回新 struct
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
}