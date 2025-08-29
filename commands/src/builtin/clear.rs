use super::Command;
use super::Cmd;

pub struct Clear;
impl Command for Clear {
    fn name(&self) -> &'static str {
        "clear"
    }
    fn run(&self, _cmd: &mut Cmd) {
        print!("\x1B[2J\x1B[H");
        use std::io::{ self, Write };
        io::stdout().flush().unwrap();
    }
}
