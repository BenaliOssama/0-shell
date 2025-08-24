use std::collections::HashMap;
use colored::*;
pub mod echo;
pub mod cd;
pub mod ls;
pub mod clear;
pub mod exit;
pub mod mkdir;
pub mod mv;
pub struct Registry {
    commands: HashMap<&'static str, Box<dyn Command>>,
}



pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[&str]);
}

impl Registry {
    pub fn new() -> Self {
        
        let mut reg = Self { commands: HashMap::new() };
        reg.register(Box::new(echo::Echo));
        reg.register(Box::new(cd::Cd));
        reg.register(Box::new(ls::Ls));
        reg.register(Box::new(clear::Clear));
        reg.register(Box::new(exit::Exit));
        reg.register(Box::new(mkdir::Mkdir));
        reg.register(Box::new(mv::Mv));
        reg
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

