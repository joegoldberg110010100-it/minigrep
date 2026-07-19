use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text: \n {}", contents);
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("\x1b[31mnot enough arguments\x1b[0m");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Config { query: query, file_path: file_path }
    }
}
