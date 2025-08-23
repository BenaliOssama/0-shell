use super::Command;
use std::env;
pub struct Cd;



impl Command for Cd {
    fn name(&self) -> &'static str { "cd" }
    fn run(&self, args: &[&str]) {
        if let Some(path) = args.first() {
            if let Err(e) = env::set_current_dir(path) {
                eprintln!("cd: {e}");
            }
        } else {
            eprintln!("cd: missing argument");
        }
    }
}