// traits 即 Cargo.toml 里的 package name
use traits::NewsArticle;
use traits::Summary;
use traits::Tweet;

use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("hourse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    //traits::notify(tweet);
    //traits::notify2(tweet);

    let words = vec![String::from("hello"), String::from("world")];
    let max_word = traits::largest(&words);
    println!("max_word: {}", max_word);

    let max_word = traits::largest_one(&words);
    println!("max_word one: {}", max_word);

}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // 每个类型都有一个叫 new 的构造方法
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/*
    要求 T 必须实现 Display + PartialOrd 这俩 trait
    才能有 cmp_display 这个方法
 */
impl<T: Display + PartialOrd> Pair<T>  {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("largest member is x = {}", self.x);
        } else {
            println!("largest member is y = {}", self.y);
        }
    }
}