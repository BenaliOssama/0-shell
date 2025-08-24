use super::Command;
use std::env;

pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, _args: &[&str]) {
        match env::current_dir() {
            Ok(path) => {
                println!("{}", path.display());
            }
            Err(e) => {
                eprintln!("pwd: error getting current directory: {}", e);
            }
        }
    }
}
