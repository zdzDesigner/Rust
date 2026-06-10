pub trait Pin<T> {}
impl<T> Pin<T> for i32 {}

pub trait MyAdd<A, B> {}
impl<A, B> MyAdd<A, B> for i32 {}



pub trait Summary {
    fn summarize(&self) -> String;
    fn about(&self) -> String {
        return format!("default {} about", self.summarize());
    }
}

// ANCHOR: here
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(t: impl Summary) {
    println!("{}", t.summarize());
}
pub fn notify2<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

#[cfg(test)]
mod notify_trait {
    #[test]
    fn trait_where() {
        println!("trait where!");
    }
}
// ANCHOR_END: here

pub mod duplicate;
pub mod inner_trait_demo;
pub mod logger;
pub mod share_demo;
pub mod supper_demo;
