use super::Command;
use std::fs;
use std::path::Path;

pub struct Mkdir;

impl Command for Mkdir {
    fn name(&self) -> &'static str {
        "mkdir"
    }

    fn run(&self, args: &[&str]) {
        for arg in args {
            let path = Path::new(arg);
            
            match fs::create_dir(path) {
                Ok(_) => println!("Directory '{}' created successfully.", arg),
                Err(e) => eprintln!("mkdir: cannot create directory '{}': {}", arg, e.to_string()),
            }
        }
    }
}
