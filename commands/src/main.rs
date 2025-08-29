
use std::io::{ self, Write,Read };
use std::fs::File;
use std::env;
use colored::*;
mod builtin;
use builtin::{ Registry,Cmd };

fn main() {
    print!("\x1B[2J\x1B[H");
    // let logo_ascii = fs::read("/home/melfihry/0-shell/src/ascii-logo.txt");
    // println!("{:?}",logo_ascii);
    // let mut file = File::open("src/ascii-logo.txt").unwrap();
    // let mut buffer = Vec::new();
    // let _ =file.read_to_end(&mut buffer);
    // for b in buffer{
    //     print!("{}",b as char)
    // }
    println!("");
    let registry = Registry::new();
    loop {
        // Uncomment this block to pass the first stage
        print!("{}{} ", build_prompt(), "$".color(Color::Yellow));
        io::stdout().flush().unwrap();
        // Wait for user input
        // io::stdin().read_line(&mut input).unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        // this will romplaced with the parser  | by fihry
        let (cmd, args) = parts.split_first().unwrap();
        let cmd_data:Cmd = Cmd::new(
            cmd.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
            Box::new(io::stdin()),
            Box::new(io::stdout()),
            Box::new(io::stderr()),
        );
        registry.run(cmd, cmd_data);
    }
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
