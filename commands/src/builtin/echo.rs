use super::Command;
pub struct Echo;
use std::io::Write;

impl Command for Echo {
    fn name(&self) -> &'static str { "echo" }
    fn run(&self, cmd: &super::Cmd) {
        cmd.stdout.as_ref().map_or_else(
            || println!("{}", cmd.args.join(" ")),
            |file| {
                let mut f = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file)
                    .unwrap();
                writeln!(f, "{}", cmd.args.join(" ")).unwrap();
            },
        );
    }
}
