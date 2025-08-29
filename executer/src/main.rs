use std::io::{self, Write};
use executer::*;
use commands::{Cmd};

fn main() {
    let cmd = Cmd::new(
        "ls".to_string(),
        vec![],                       // empty args
        Box::new(io::stdin()),         // stdin
        Box::new(io::stdout()),       // stdout
        Box::new(io::stderr()),       // stderr
    );

    let err = exec(cmd, "/usr/bin");
    println!("{:?}", err);
}
