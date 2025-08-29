use commands::{ Cmd, Registry };
// take a command,
// excute the command
use std::io::Write;
use std::fmt::Debug;
use std::fmt::Formatter;

pub fn excute(cmd: Cmd) {
    let registry = Registry::new();

    registry.run( cmd);
    // check if command exist in register
    // check if command exist in file of binary
    // if not write error to the standard error

    // excute the command someway if it is builtin
    // excute the command someway if it is internal binary
}
