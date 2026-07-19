# minigrep
 
A learning project in Rust — a simplified `grep` clone, built while working through the official **"The Rust Programming Language"** book (Chapter 12).
 
## Run
 
```bash
cargo run -- <query> <file_path>
```
 
Example:
 
```bash
cargo run -- the poem.txt
```
 
## What this covers
 
Practicing idiomatic Rust error handling — moving from `panic!` to `Result`:
 
- Parsing CLI args into a `Config` struct via `Config::build`, returning `Result<Config, &'static str>` instead of panicking on bad input.
- Handling that `Result` in `main` with `unwrap_or_else`, printing a clean error and exiting via `process::exit(1)`.
- Extracting core logic into `run(config: Config) -> Result<(), Box<dyn Error>>`.
- Using `?` instead of `.expect(...)` for file reading.
- `Box<dyn Error>` for returning different error types through one interface.
- Handling `run`'s result in `main` with `if let Err(e) = ...`.
## Source
 
Based on Chapter 12 of [The Rust Book](https://doc.rust-lang.org/book/) — "An I/O Project: Building a Command Line Program".
 
## Status
 
Work in progress, following along with the chapter.
 

