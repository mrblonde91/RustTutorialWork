use std::fs;
use std::error::Error;

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    //question mark returns the value for us to handle
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
}

pub struct Config {
    filename: String,
    query: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{filename, query})
    }
}

pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        println!("{}",line);
        if line.contains(query){
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}