use std::collections::HashMap;
use colored::*;
use std::io::{ Write, Read };
use std::fmt::Formatter;
use std::fmt::Debug;

pub mod echo;
pub mod cd;
pub mod clear;
pub mod exit;

pub struct Registry {
    commands: HashMap<&'static str, Box<dyn Command>>,
}

use crate::builtin::{ echo::Echo, cd::Cd, clear::Clear, exit::Exit };

pub struct Cmd {
    cmd: String,
    args: Vec<String>,
    stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl Debug for Cmd{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "name: {}", self.cmd)?;
        write!(f, "args: {:?}", self.args)?;
        Ok(())
    }
}

impl Cmd {
    pub fn new(
        cmd: String,
        args: Vec<String>,
        stdin: Box<dyn Read>,
        stdout: Box<dyn Write>,
        stderr: Box<dyn Write>
    ) -> Self {
        Self { cmd, args, stdin, stdout, stderr }
    }
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &mut Cmd);
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

    pub fn run(&self,  &mut cmd_data: Cmd) {
        if let Some(cmd) = self.commands.get(cmd_data.cmd.as_str()) {
            cmd.run(&mut cmd_data);
        } else {
            eprintln!(
                "{} command not found: {}",
                "0-shell".color(Color::BrightRed),
                cmd_data.cmd.red().bold()
            );
        }
    }
}
