use std::cmp::PartialOrd;

// Copy trait이 뭐임 ????? 안쓰면 에러남,,
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// Copy안쓰고 참조자를 반환해서 하는 방법??

pub trait Summarizable {
    const KIND: string;
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("{}: (Read more from {}...)", self::KIND, self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    const KIND: string = "NewsArticle";

    fn author_summary(&self) -> String {
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
    const KIND: string = "Tweet";

    fn author_summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// Summarizable을 구현한 타입만 허용됨. 공변적?
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}


// // 타입스크립트처럼 제네릭으로 특정 구조체를 받는건 안되나..?
// pub fn tweetNotify2<T: Tweet>(item: T) {
//     println!("Breaking news! {}", item.summary());
// }

// // 여러개의 Trait 구현한 타입들을 받을때
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

// // 보기 쉽도록 where 절을 사용할 수 있음
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}