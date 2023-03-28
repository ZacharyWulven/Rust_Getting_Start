use std::{fmt::{Display, Debug}, error::Error};
use core::fmt::Formatter;

// 定义一个 trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    //fn summarize(&self) -> String;
    // 默认实现
    fn summarize(&self) -> String {
        format!("Read more by {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // 注释掉，就用默认实现了
    // fn summarize(&self) -> String {
    //     format!("{} -> {}", self.username, self.content)
    // }
}


// impl trait 写法
pub fn notify(item: impl Summary + Display) {
    println!("Breaking news {}", item.summarize());
}

// trait bound 写法
pub fn notify2<T: Summary + Display>(item: T) {
    println!("Breaking news {}", item.summarize());
}


pub fn notify_no_where<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("where {}", a.summarize())
}

pub fn notify_where<T, U>(a: T, b: U) -> String
    where T: Summary + Display, 
          U: Clone + Debug  {
    format!("where {}", a.summarize())
}