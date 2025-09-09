pub use commands::Cmd;
use commands::Registry;
use std::env;
use std::error::Error;
use std::process::Command;

// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/?utm_source=chatgpt.com

pub fn exec(cmd: Cmd) {
    let registry = Registry::new();

    // Built-in
    if registry.has(&cmd) {
        registry.run(cmd);
        return;
    }

    // External command (search in PATH)
    let status = Command::new(&cmd.cmd)
        .args(&cmd.args)
        .spawn()
        .and_then(|mut child| child.wait());

    match status {
        Ok(status) => {
            if !status.success() {
                eprintln!("command '{}' exited with {:?}", cmd.cmd, status);
            }
        }
        Err(e) => {
            eprintln!("failed to execute '{}': {}", cmd.cmd, e);
        }
    }
}
