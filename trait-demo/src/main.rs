// use trait_demo::Sumary;
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Sumary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }

//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Sumary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("{}", self.username)
//     }

//     fn summarize(&self) -> String {
//         format!("{}: {} ", self.username, self.content)
//     }
// }

// fn notify<T: Sumary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn returns_summarizable() -> impl Sumary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("aladdin"),
//         content: String::from("i miss you"),
//         reply: false,
//         retweet: true,
//     };
//     let b = tweet.summarize();
//     let c = returns_summarizable();
//     notify(c)
//     // println!("{}", b);
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(3, 4);
    pair.cmp_display();
}
