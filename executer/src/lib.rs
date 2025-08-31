use std::error::Error;
use std::env;
use std::process::Command;
use commands::Registry;
pub use commands::Cmd;

// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/?utm_source=chatgpt.com

pub fn exec(cmd: Cmd) -> Result<(), Box<dyn Error>> {
    let registry = Registry::new();

    // Built-in
    if registry.has(&cmd) {
        registry.run(cmd);
        return Ok(());
    }

    // External in env_dir
    let path = env::var(&cmd.cmd)?;

    let status = Command::new(path).args(cmd.args).spawn()?.wait()?;

    if !status.success() {
        return Err(format!("command '{}' exited with {:?}", cmd.cmd, status).into());
    } 
    Ok(()) 
}
