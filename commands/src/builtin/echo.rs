use super::Command;
use super::Cmd;
pub struct Echo;
use std::io::Write;

impl Command for Echo {
    fn name(&self) -> &'static str { "echo" }
    fn run(&self, cmd: &mut Cmd) {
        match cmd.args.get(0).map(|s| s.as_str()) {
            Some("-n") => {
                cmd.args.remove(0);
                cmd.stdout.write_all(format!("{}$\n", cmd.args.join(" ")).as_bytes()).unwrap();
            }
            _ => {
                cmd.stdout.write_all(format!("{}\n", cmd.args.join(" ")).as_bytes()).unwrap();
            }
        }
    }
}
