use std::env;
use std::fs;
//12.01 minigrep
//use std::env::args_os if not valid unicode
// collect arguments from the iterator
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename)
    .expect("The file does not exist");
    println!("Contents of file:\n {}", contents);
}
