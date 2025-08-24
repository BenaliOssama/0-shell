use super::Command;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Mv;

impl Command for Mv {
    fn name(&self) -> &'static str {
        "mv"
    }

    fn run(&self, args: &[&str]) {
        if args.len() < 2 {
            let err = if args.len() == 1 {
                format!("mv: missing destination file operand after '{}'", args[0])
            } else {
                "mv: missing file operand".to_owned()
            };
            eprintln!("{}", err);
            return;
        }
        let mut i = 0;
        while i < args.len() - 1 {
            let src = Path::new(args[i]);
            let dst = Path::new(args[args.len() - 1]);

            let final_dst: PathBuf = if dst.is_dir() {
                dst.join(src.file_name().unwrap_or_default())
            } else {
                dst.to_path_buf()
            };

            match fs::rename(src, &final_dst) {
                Ok(_) => {},
                Err(e) => eprintln!("mv: cannot move '{}': {}", args[i], e)
            }
            i += 1;
        }
    }
}
