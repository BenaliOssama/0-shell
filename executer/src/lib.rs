// take a command,
// excute the command
use std::io::Write;

struct Command {
    cmd: String,
    args: Vec<String>,
    stdin: Option<Box<dyn Write>>,
    stdout: Option<Box<dyn Write>>,
    stderr: Option<Box<dyn Write>>,
}


pub fn excute(cmd: Command) {
    // check if command exist in register
    // check if command exist in file of binary
    // if not write error to the standard error

    // excute the command someway if it is builtin
    // excute the command someway if it is internal binary
}
