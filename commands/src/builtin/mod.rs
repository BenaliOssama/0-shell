use std::collections::HashMap;
use colored::*;
pub mod echo;
pub mod cd;
// pub mod ls;
pub mod clear;
pub mod exit;
pub struct Registry {
    commands: HashMap<&'static str, Box<dyn Command>>,
}
pub struct Cmd {
    pub name: String,
    pub args: Vec<String>,
    pub bg: bool,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &Cmd);
}

impl Registry {
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name(), cmd);
    }

    pub fn run(&self, name: &str, cmd_data: Cmd) {
        if let Some(cmd) = self.commands.get(name) {
            cmd.run(&cmd_data);
        } else {
            eprintln!(
                "{} command not found: {}",
                "0-shell".color(Color::BrightRed),
                name.red().bold()
            );
        }
    }
}
