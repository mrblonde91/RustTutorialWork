fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

//Lifetime annotations define relationships, it makes sure that they must live as long as the lifetime of 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
