//10.3 Lifetimes
use std::fmt::Display;

fn main() {
    //string literals have a lifetime that's fixed and always available
    let s: &'static str = "I have a static lifetime.";

    let novel = String::from("Call me ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt{part:first_sentence};
    println!("Hold on: {}", i.part);
    let hello = String::from("hello");
    let world = String::from("worlds");
    let result = longest_with_an_announcement(&hello, &world, i.part);
}

struct ImportantExcerpt<'a>{
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a>{
    //reference to self so dont need to reference lifetime
    fn level(&self) -> i32 {
        3
    }

    fn annouce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part
    }
}

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display{
        println!("Announcemnt! {}", ann);
        if x.len() > y.len(){
            x
        }else{
            y
        }
    }