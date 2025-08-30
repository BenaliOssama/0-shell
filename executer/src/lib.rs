use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use commands::Registry;
pub use commands::Cmd;




// let dir = PathBuf::from(env_dir);
//
// if !dir.exists() {
//     return Err(format!("directory '{}' not found", dir.display()).into());
// }
//
// let path = env::var(&cmd.cmd)?;
//
// let status = Command::new(path).args(cmd.args).spawn()?.wait()?;
//
// if !status.success() {
//     return Err(format!("command '{}' exited with {:?}", cmd.cmd, status).into());
// } 
// Ok(()) 


pub fn execute(cmd: Cmd, env_dir: &str) -> Result<(), Box<dyn Error>> {
    let registry = Registry::new();

    // 1. Built-in
    if registry.has(&cmd) {
        registry.run(cmd);
        return Ok(());
    }

    // 2. External in env_dir
    let dir = PathBuf::from(env_dir);

    if !dir.exists() {
        return Err(format!("directory '{}' not found", dir.display()).into());
    }

    let entries = fs::read_dir(&dir)?;

    let mut found_path = None;
    for entry in entries {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename == cmd.cmd {
                found_path = Some(entry.path());
                break;
            }
        }
    }

    if let Some(path) = found_path {
        let status = Command::new(path).args(cmd.args).spawn()?.wait()?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("command '{}' exited with {:?}", cmd.cmd, status).into())
        }
    } else {
        Err(format!("command not found: {}", cmd.cmd).into())
    }
}
