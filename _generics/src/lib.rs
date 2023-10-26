use std::fmt::{Display, Debug};
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline,self.author,self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet:bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify1<T: Summary + Display>(item1: T) {
    println!("Breaking news! {}", item1.summarize());
}

pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U)-> String {
    format!("Breaking news! {}", a.summarize())
}


pub fn notify2<T, U>(a: T, b: U)-> String 
where T: Summary + Display,
        U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())

}

pub fn notify3(s: &str) -> impl Summary{
    NewsArticle{
        headline: String::from("Rust is the best language"),
        content: String::from(format!("{}", s)),
        author: String::from("The Rust Team"),
        location: String::from("Everywhere"),
    }

}

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}