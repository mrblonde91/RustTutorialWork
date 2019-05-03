fn main() {
    let mut s = String::from("How now brown cow?");

    let hello = first_word(&s); //Immutable borrow

    //modifying s will result in a failure as hello has a reference to it.
    
    println!("{}",hello);
}

fn first_word(s: &String) -> &str{
    let byte = s.as_bytes();

    for(index, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[..index];// Don't have to specify start index if starting at 0
        }
    }

    &s[..]
}