pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notifyTwo<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news: {} and {}", item1.summarize(), item2.summarize());
}

pub fn notifyTwo2(item1: &impl Summary, item2: &impl Summary) {
    notify(item1);
    notify(item2);
}

use std::fmt::Display;

pub fn notify_and_display(item: &(impl Summary + Display)) {
    // ...
}

pub fn notify_and_display2<T: Summary + Display>(item: &T) {
    // ... 
}

use std::fmt::Debug;

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {
    // ...
}

pub fn some_function2<T, U> (t: T, u: U)
    where T: Display + Clone,
          U: Clone + Debug,
{
    // ...
}

pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news: {}", item.summarize());
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     // println!("1 new tweet: {}", tweet.summarize());
//     notify(&tweet);

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };

//     // println!("New article available! {}", article.summarize());
//     notify(&article);


// }



// use std::fmt::Display;

struct Point <T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p = Point::new(10.5, 15.6);
    p.cmp_display();
}