use super::Command;
use std::env;
pub struct Cd;
use super::Cmd;



impl Command for Cd {
    fn name(&self) -> &'static str { "cd" }
    fn run(&self, cmd: &Cmd) {
        if let Some(path) = cmd.args.get(0) {
            if let Err(e) = env::set_current_dir(path) {
                cmd.stderr.as_ref().map_or_else(|| eprintln!("cd: {}: {}", path, e), |file| {
                    let mut f = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file)
                        .unwrap();
                    use std::io::Write;
                    writeln!(f, "cd: {}: {}", path, e).unwrap();
                });
            }
        } else {
            eprintln!("cd: missing argument");
        }
    }
}