use std::collections::HashMap;
use colored::*;
pub mod echo;
pub mod cd;
pub mod ls;
pub mod clear;
pub struct Registry {
    commands: HashMap<&'static str, Box<dyn Command>>,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[&str]);
}

impl Registry {
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name(), cmd);
    }

    pub fn run(&self, name: &str, args: &[&str]) {
        if let Some(cmd) = self.commands.get(name) {
            cmd.run(args);
        } else {
            eprintln!("{} command not found: {}", "0-shell".color(Color::BrightRed),name.red().bold());
        }
    }
}

