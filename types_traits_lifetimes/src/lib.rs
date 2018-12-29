pub trait Summary {
    fn summarize(&self) -> String;
    fn article_type(&self) -> String;

    fn read_more(&self) -> String {
        "Read more.....".to_string()
    }
    fn paywall(&self) -> String {
        format!("Pay us if you wanna read this {}...", self.article_type())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn article_type(&self) -> String {
        "News Article".to_string()
    }
    fn summarize(&self) -> String {
        format!(
            "headline: {}, author: {}, location: {}",
            self.headline, self.author, self.location
        )
    }
}

pub struct BlogArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn article_type(&self) -> String {
        "Tweet".to_string()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait SayIt {
    fn the_call(&self) -> String;
}

pub struct Mituo {
    pub shout: String,
}

impl SayIt for Mituo {
    fn the_call(&self) -> String {
        self.shout.to_string()
    }
}

pub struct Senda {
    pub shout: String,
}

impl SayIt for Senda {
    fn the_call(&self) -> String {
        self.shout.to_string()
    }
}

pub struct NahaNaha {
    pub shout: String,
}

impl NahaNaha {
    pub fn the_call(&self) -> String {
        self.shout.to_string()
    }
}

pub struct ShoutOut<T> {
    pub obj: T,
}

impl<T> ShoutOut<T> {
    pub fn kimoi(&self) -> String {
        "デュフフフフ".to_string()
    }
}

impl<T> ShoutOut<T>
where
    T: SayIt,
{
    pub fn shoutout(&self) -> String {
        "せんだ、みつお、なはなは".to_string()
    }
}
