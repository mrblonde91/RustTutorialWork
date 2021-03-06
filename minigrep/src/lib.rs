use std::fs;
use std::error::Error;
use std::env;

//$env:CASE_INSENSITIVE = 1 to make case insensitive
pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    //question mark returns the value for us to handle
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in result{
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{filename, query, case_sensitive})
    }
}

pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    
    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
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

    #[test]
       fn case_insensitive(){
        let query = "SAFE";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}