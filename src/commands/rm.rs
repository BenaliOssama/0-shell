use super::Command;
use std::fs;
use std::path::Path;

pub struct Rm;

impl Command for Rm {
    fn name(&self) -> &'static str {
        "rm"
    }

    fn run(&self, args: &[&str]) {
        if args.is_empty() {
            eprintln!("rm: missing operand");
            return;
        }

        let (recur, targets): (bool, &[&str]) = if args[0] == "-r" {
            if args.len() < 2 {
                eprintln!("rm: missing operand after '-r'");
                return;
            }
            (true, &args[1..])
        } else {
            (false, args)
        };

        for target in targets {
            let path = Path::new(target);

            let result = if path.is_dir() {
                if recur {
                    fs::remove_dir_all(path)
                } else {
                    eprintln!("rm: cannot remove '{}': Is a directory", target);
                    continue;
                }
            } else {
                fs::remove_file(path)
            };

            if let Err(e) = result {
                eprintln!("rm: cannot remove '{}': {}", target, e);
            }
        }
    }
}
