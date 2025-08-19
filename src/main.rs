#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;


fn main() {
     loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        io::stdin().read_line(&mut input);
        
        if input.trim() == "exit" {
            process::exit(1);
        }
        println!("{}: command not found", input.trim());
     }
}
