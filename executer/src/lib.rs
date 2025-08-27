use commands::*;
// take a command,
// excute the command
use std::io::Write;
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct Command {
    name: String,
    args: Vec<String>,
    stdin: Box<dyn Write>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl Command {
    // command
    pub fn new(
        name: String,
        args: Vec<String>,
        stdin: Box<dyn Write>,
        stdout: Box<dyn Write>,
        stderr: Box<dyn Write>
    ) -> Self {
        Self { name, args, stdin, stdout, stderr }
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "name: {}", self.name)?;
        write!(f, "args: {:?}", self.args)?;
        Ok(())
    }
}

pub fn excute(cmd: Command) {
    // check if command exist in register
    // check if command exist in file of binary
    // if not write error to the standard error

    // excute the command someway if it is builtin
    // excute the command someway if it is internal binary
}
