use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        println!("\x1b[41m[!]Application error: {e}\x1b[0m");
        process::exit(1);
    };
}
//==================================================================
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n {}", contents);
    Ok(())
}
//===================================================================
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("\x1b[41m[!!!] Not enough arguments!\x1b[0m");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query: query, file_path: file_path })
    }
}
