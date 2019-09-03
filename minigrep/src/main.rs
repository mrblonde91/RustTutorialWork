use std::env;
use std::fs;
//12.01 minigrep
//use std::env::args_os if not valid unicode
// collect arguments from the iterator
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let contents = fs::read_to_string(config.filename)
    .expect("The file does not exist");
    println!("Contents of file:\n {}", contents);
}

struct Config {
    filename: String,
    query: String,
}

impl Config{
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{filename, query}
}
}