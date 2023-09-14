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

pub fn notify_return(flag: bool) -> impl Summary {
    Tweet {
        username: String::from("username"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

pub fn notify_return2(flag: bool) -> impl Summary {
    // 只能返回一个实现了 Summary 的类型
    NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    }

    // 如果实现返回多个类型，会报错
    /* 
    if flag {
        Tweet {
            username: String::from("username"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    } else {
        NewsArticle {
            headline: String::from("headline"),
            location: String::from("location"),
            author: String::from("author"),
            content: String::from("content"),
        }
    }
    */
}


// std::cmp::PartialOrd 在 prelude 模块里，所以不需要导入
pub fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    // 这里报错需要加上 Copy trait，但字符串无法使用所以可以用 Clone trait
    let mut largest = list[0].clone(); 
    for item in list.iter() {
        // 只要实现了 std::cmp::PartialOrd 这个 trait，那么 T 才能使用 > 进行比较
        if item > &largest { 
            largest = item.clone();
        }
    }
    largest
}


// std::cmp::PartialOrd 在 prelude 模块里，所以不需要导入
pub fn largest_one<T: PartialOrd>(list: &[T]) -> &T {
    // 这里报错需要加上 Copy trait，但字符串无法使用所以可以用 Clone trait
    let mut largest = &list[0]; 
    for item in list.iter() {
        // 只要实现了 std::cmp::PartialOrd 这个 trait，那么 T 才能使用 > 进行比较
        if item > &largest { 
            largest = item;
        }
    }
    largest
}


