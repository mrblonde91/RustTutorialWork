## Rust tutorials
Based on tutorials from https://doc.rust-lang.org/book/ 

# Ownership
* Ownership doesn't apply to passing references or slices
* Passing the variable directly and not as a reference, will pass ownership on and unless it's returned, you won't regain ownership.
*  Passing a reference is classified as borrowing, you get it back after done.
* Unless explicitly done, variables when mapped to another variable have their place in the heap dropped. Does not apply to more primitive types such as int. Sample below.

    let s1 = String::from("hello");
    <br/>let s2 = s1; 
    <br/>println!("{}, world!", s1);

* Clone can be used to explicitly retain two versions. Sample below.

    let s1 = String::from("hello");
    <br/>let s2 = s1.clone();
    <br/>println!("s1 = {}, s2 = {}", s1, s2);

## Slices
* Ownership doesn't apply. 
* Sample slice below

     let s = String::from("hello world");  <br/>let hello = &s[0..5];<br/>let world = &s[6..11];

* Note: using a literal param such as &str is better than &String as it will allow both String and str.
 
 