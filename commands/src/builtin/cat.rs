use super::Command;
use std::fs::File;
use std::io::{self};

pub struct Cat;

impl Command for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn run(&self, args: &[&str]) {
        if args.is_empty() {
            return;
        }

        for filename in args {
            match File::open(filename) {
                Ok(mut file) => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();

                    if let Err(e) = io::copy(&mut file, &mut handle) {
                        eprintln!("cat: error reading '{}': {}", filename, e);
                    }
                }
                Err(e) => eprintln!("cat: {}: {}", filename, e),
            }
        }
    }
}
