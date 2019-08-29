use std::fmt::Display;
// 10.2 Traits
fn main() {
    let tweet = Tweet{
        username: String::from("Mike"),
        content: String::from("Mike was here"),
        reply: false,
        retweet:false
    };
    println!("1 new tweet:{}", tweet.summarize());
    notify(tweet);
}

//Must be passed type that implements summary
//Called trait bound syntax, could also be written as (item: impl Summary)
pub fn notify<T: Summary>(item: T){
    println!("Breaking news {}", item.summarize())
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y:T) -> Self{
        Self{
            x,
            y,
        }
    }
}

//Conditional implementation
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is x: {}", self.x);
        }else{
             println!("The largest member is y: {}", self.y);
        }
    }
}

//Also possible to bind by more than one trait
//pub fn notify(item: impl Summary + Display) {
//pub fn notify<T: Summary + Display>(item: T) {
//Where clause can be used to make this cleaner
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

//Can also define a return type that implements trait
//Particularly useful in iterators or closurse
// fn returns_summarizable(switch: bool)->impl Summary{
//    if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

pub trait Summary{
    fn summarize_author(&self)->String;
    //Default summarize if no implementation
    fn summarize(&self) ->String{
        format!("(Read more) from {}", self.summarize_author())
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) ->String{
        format!("@{}", self.author)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//To use default, leave this blank.
impl Summary for Tweet{
    // fn summarize(&self) -> String{
    //     format!("{}:{}", self.username, self.content)
    // }
    fn summarize_author(&self) ->String{
        format!("@{}", self.username)
    }
}