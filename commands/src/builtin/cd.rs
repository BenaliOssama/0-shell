use super::Command;
use std::env;
pub struct Cd;
use super::Cmd;



impl Command for Cd {
    fn name(&self) -> &'static str { "cd" }
    fn run(&self, cmd: &mut Cmd) {
        if let Some(path) = cmd.args.get(0) {
            if let Err(e) = env::set_current_dir(path) {
                cmd.stderr.write_all(format!("cd: {}: {}\n", path, e).as_bytes()).unwrap();
            }
        } else {
            cmd.stderr.write_all(b"cd: missing argument\n").unwrap();
        }
    }
}