fn main() {
    let tweet = Tweet {
        username: String::from("horse_books"),
        content: String::from("of course, ..."),
        reply: false,
        retweet: false,
    };
    println!("new tweet: {}", tweet.summarize());

    let pair = Pair::new(10, 20);
    pair.cmp_display();
}

trait Summary {
    // 默认实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

use std::fmt::format;
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

use std::fmt::Display;
fn trait_func(item: impl Summary + Display) {}

fn trait_func2<T: Summary + Display>(item: T) {}

fn return_trait() -> impl Summary {
    Tweet {
        username: String::from("horse_books"),
        content: String::from("of course, ..."),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest member: {}", self.x);
        } else {
            println!("largest member: {}", self.y);
        }
    }
}
