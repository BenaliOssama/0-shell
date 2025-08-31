use std::error::Error;
use std::env;
use std::process::Command;
use commands::Registry;
pub use commands::Cmd;

pub fn exec(cmd: Cmd) -> Result<(), Box<dyn Error>> {
    let registry = Registry::new();

    // 1. Built-in
    if registry.has(&cmd) {
        registry.run(cmd);
        return Ok(());
    }

    // 2. External in env_dir
    let path = env::var(&cmd.cmd)?;

    let status = Command::new(path).args(cmd.args).spawn()?.wait()?;

    if !status.success() {
        return Err(format!("command '{}' exited with {:?}", cmd.cmd, status).into());
    } 
    Ok(()) 
}
