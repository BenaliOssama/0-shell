use std::error::Error;
use std::env;
use std::process::{Child, Command, Stdio};
use std::path::{Path, PathBuf};

pub use commands::{Cmd, Registry};

pub fn exec(commands: Vec<Cmd>) -> Result<(), Box<dyn Error>> {
    let mut prev_stdout = None;
    let mut children: Vec<Child> = Vec::new();

    let mut cmd_iter = commands.into_iter().peekable();

    while let Some(cmd) = cmd_iter.next() {
        let registry = Registry::new();
        if registry.has(&cmd) {
            // Handle built-in commands (no piping)
            if cmd_iter.peek().is_some() || prev_stdout.is_some() {
                return Err("Built-in commands cannot be used in pipelines".into());
            }
            registry.run(cmd);
            return Ok(());
        }

        // Find the executable in PATH
        let executable = find_executable(&cmd.cmd)
            .ok_or_else(|| format!("command not found: {}", cmd.cmd))?;

        let stdin = match prev_stdout.take() {
            Some(output) => Stdio::from(output),
            None => Stdio::inherit(),
        };

        let stdout = if cmd_iter.peek().is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };

        let mut child = Command::new(&executable)
            .args(&cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?;

        prev_stdout = child.stdout.take();
        children.push(child);
    }

    // Wait for all children to finish
    for mut child in children {
        child.wait()?;
    }

    Ok(())
}

/// Finds the executable path using the PATH environment variable.
fn find_executable(cmd: &str) -> Option<PathBuf> {
    let dir = env::var("DIR").ok()?; // get env var, return None if missing
    let candidate = Path::new(&dir).join(cmd); // join paths safely
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}