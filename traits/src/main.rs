// traits 即 Cargo.toml 里的 package name
use traits::NewsArticle;
use traits::Summary;
use traits::Tweet;

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

}

