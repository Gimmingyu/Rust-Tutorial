extern crate traits;

use traits::*;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summary());
    println!("1 new tweet: {}", tweet.example());

    notify(tweet);
}
