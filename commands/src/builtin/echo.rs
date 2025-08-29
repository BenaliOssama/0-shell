use super::Command;
use super::Cmd;
pub struct Echo;
use std::io::Write;

impl Command for Echo {
    fn name(&self) -> &'static str { "echo" }
    fn run(&self, cmd: &mut Cmd) {
        writeln!(&mut cmd.stdout, "{}", cmd.args.join(" ")).unwrap();
    }
}
