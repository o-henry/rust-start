pub trait Summary {
    fn summarize(&self) -> String; // 트레이트를 구현할 타입의 행위를 설명하는 메서드 시그니처
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
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
        format("{}: {}", self.username, self.content)
    }
}

let tweet = Tweet { 
    username: String::from("horse_ebooks"),
    content: String::from("러스트 언어 공부를 시작했습니다."),
    reply: false,
    retweet: false,
};

println!("새 트윗 1개: {}", tweet.summarize());