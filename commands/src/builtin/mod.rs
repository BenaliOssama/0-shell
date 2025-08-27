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
use crate::builtin::{ echo::Echo, cd::Cd,clear::Clear,exit::Exit };
pub struct Cmd {
    pub name: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &Cmd);
}

impl Registry {
    pub fn new() ->  Self {
        let mut  register = Registry {
            commands: HashMap::new(),
        };
        register.register(Box::new(Echo));
        register.register(Box::new(Cd));
        register.register(Box::new(Clear));
        register.register(Box::new(Exit));
        register
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
