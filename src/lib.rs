use std::fs;
use std::error::Error;
use std::env;
//===========================FUNCTION=====================================
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case == true {
        for line in search_case_insensitive(&config.query, &contents){
            println!("\x1b[42m{}\x1b[0m", line);
        }
    }else {
        for line in search(&config.query, &contents) {
            println!("\x1b[42m{}\x1b[0m", line);
        }
    }
    
    Ok(())
}
//============================
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}
//===========================
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
        result
}
//==============================STRUCT===================================
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case:bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        if args.len() < 3 {
            return Err("\x1b[41m[!!!] Not enough arguments!\x1b[0m");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query: query, file_path: file_path, ignore_case: ignore_case })
    }
}
//=================================TEST================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(&query, &contents));
    }
}
