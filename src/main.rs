use colored::*;
use std::env;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufReader, Read, Write};
mod commands;
use commands::Registry;

fn main() {
    // print!("\x1B[2J\x1B[H");

    match File::open("src/ascii-logo.txt") {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            let _ = f.read_to_end(&mut buffer);
            for b in buffer {
                print!("{}", b as char)
            }
            println!("");
        },
        Err(_e) => {},
    }

    let registry = Registry::new();


    loop {
        // Uncomment this block to pass the first stage
        print!("{}{} ", build_prompt(), "$".color(Color::Yellow));
        io::stdout().flush().unwrap();
        // Wait for user input
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        // this will romplaced with the parser  | by fihry
        let (cmd, args) = parts.split_first().unwrap();
        registry.run(cmd, args);
    }
}

pub fn build_prompt() -> String {
    let user = env::var("USER").unwrap_or("user".to_string());
    let cwd = env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or("?".to_string());
    format!(
        "{}:{}::[{}]",
        "0-shell".bright_green().bold(),
        user.on_bright_white(),
        cwd.bright_blue()
    )
}
