use std::collections::HashMap;
use colored::*;
use std::io::Write;

pub mod echo;
pub mod cd;
// pub mod ls;
pub mod clear;
pub mod exit;

pub struct Registry {
    commands: HashMap<&'static str, Box<dyn Command>>,
}

use crate::builtin::{ echo::Echo, cd::Cd, clear::Clear, exit::Exit };

pub struct Cmd {
    cmd: String,
    args: Vec<String>,
    stdin: Box<dyn Write>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl Cmd {
    // command
    fn new(
        name: String,
        args: Vec<String>,
        stdin: dyn Write,
        stdout: dyn Write,
        stderr: dyn Write
    ) -> Self {
        Self { name, args, stdin, stdout, stderr }
    }
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &Cmd);
}

impl Registry {
    pub fn new() -> Self {
        let mut register = Registry {
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
