use super::Command;

pub struct Clear;

impl Command for Clear {
    fn name(&self) -> &'static str { "clear" }
    fn run(&self, _args: &[&str]) {
        print!("\x1B[2J\x1B[H");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }
}
