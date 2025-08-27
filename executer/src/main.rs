use std::io::{self, Write};
use executer::{Command};
use command::{Cmd};

fn main() {
    let cmd = Command::new(
        "pwd".to_string(),
        vec![],                       // empty args
        Box::new(io::sink()),         // stdin
        Box::new(io::stdout()),       // stdout
        Box::new(io::stderr()),       // stderr
    );

    println!("{:?}", cmd);
}
