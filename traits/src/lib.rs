pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Example for Tweet {
    fn example(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summarizable {
    fn summary(&self) -> String;
    
    // fn example(&self) -> String {
    //     String::from("(Read more...)")
    // }
}

pub trait Example {
    fn example(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}