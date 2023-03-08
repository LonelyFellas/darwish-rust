pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("article{}, author is {}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} posts a article{}", self.username, self.content)
    }
}

fn main () {}