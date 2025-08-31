use std::io::{ self, Write };
use ctrlc;
use evaluating::evaluate;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::env;
use colored::*;
// use std::io::stdin;
// use termion::{event::Key, input::TermRead};

fn main() {
    ctrlc
        ::set_handler(move || {
            println!("");
        })
        .expect("Error setting Ctrl-C handler");
    print!("\x1B[2J\x1B[H");
    let mut file = File::open("ascii-logo.txt").unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    for b in buffer {
        print!("{}", b as char);
    }
    println!("");
    loop {
        print!("{}{} ", build_prompt(), "$".color(Color::Yellow));
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match read_input(&mut input) {
            Ok(0) => std::process::exit(0),
            Ok(_) => evaluate(&input),
            Err(e) => eprint!("{}", e),
        }
    }
}

fn read_input(input: &mut String) -> Result<usize, std::io::Error> {
    let mut i: Result<usize, std::io::Error>;
    loop {
        let mut line = String::new();
        i = io::stdin().read_line(&mut line);
        if let Ok(x) = i && x == 0 {
            std::process::exit(0);
        }
        input.push_str(&line);
        if quotes_even(&input) {
            break;
        } else {
            print!(">");
            io::stdout().flush().unwrap();
        }
    }
    i
}

fn quotes_even(input: &str) -> bool {
    let double_quotes = input.matches('"').count();
    let single_quotes = input.matches('\'').count();

    double_quotes % 2 == 0 && single_quotes % 2 == 0
}
pub fn build_prompt() -> String {
    let user = env::var("USER").unwrap_or("user".to_string());
    let cwd = env
        ::current_dir()
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