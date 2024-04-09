pub trait Summary {
    fn summarize(&self) -> String;

    fn get_author(&self) -> String;

    fn default_summarize(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }
}

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

    fn get_author(&self) -> String {
        format!("@{}", self.author)
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

    fn default_summarize(&self) -> String {
        self.summarize()
    }

    fn get_author(&self) -> String {
        format!("@{}", self.username)
    }
}
