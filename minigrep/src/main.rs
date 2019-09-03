use std::env;
use std::process;
use minigrep::Config;

//12.01 minigrep
//use std::env::args_os if not valid unicode
// collect arguments from the iterator
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        //eprintln prints to standard errors
        eprintln!("Problems parsing arguments: {}", err);
        process::exit(1);
    });

    //we only care if there's an error
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}