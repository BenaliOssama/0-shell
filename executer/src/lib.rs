use std::error::Error;
use std::env;
use std::process::{ Child, Command, Stdio };
pub use commands::{ Registry, Cmd };

pub fn exec(commands: Vec<Cmd>) -> Result<(), Box<dyn Error>> {
    let mut prev_stdout = None; // This will hold the output of the previous command
    let mut children: Vec<Child> = Vec::new(); // This will hold all child processes we spawn

    let cmd_iter = commands.into_iter(); // Create an iterator from the Vec<Cmd>

    // Process each command in the pipeline
    let mut cmd_iter = cmd_iter.peekable();

    while let Some(cmd) = cmd_iter.next() {
        // Look for the command in the registry first
        let registry = Registry::new();
        if registry.has(&cmd) {
            registry.run(cmd);
            return Ok(()); // If registry has the command, run it and return
        }

        let path = "bin";//env::var(&cmd.cmd)?;
        // External command: get path from the environment or use cmd.cmd directly
        // let path = env::var(&cmd.cmd).unwrap_or(cmd.cmd.clone());

        // Input: either from previous command's output or inherit from shell
        let stdin = match prev_stdout.take() {
            Some(output) => Stdio::from(output),
            None => Stdio::inherit(),
        };

        // Output: pipe to next command if there is one, otherwise inherit
        let stdout = if cmd_iter.peek().is_some() {
            Stdio::piped() // More commands follow, so pipe output
        } else {
            Stdio::inherit() // Last command, output to terminal
        };

        // Spawn the command with configured stdin/stdout
        let mut child = Command::new(path).args(cmd.args).stdin(stdin).stdout(stdout).spawn()?;

        // Take ownership of stdout for the next command in the pipeline
        prev_stdout = child.stdout.take();
        children.push(child);
    }

    // Wait for all spawned children to exit
    for mut child in children {
        child.wait()?;
    }

    Ok(())
}
