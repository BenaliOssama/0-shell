use super::Command;
use colored::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::env;
pub struct Ls;

impl Command for Ls {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn run(&self, args: &[&str]) {
        let mut show_all = false;
        let mut long = false;
        let mut classify = false;
        let mut paths: Vec<&str> = Vec::new();

        for arg in args {
            match arg as &str {
                "-a" => {
                    show_all = true;
                }
                "-l" => {
                    long = true;
                }
                "-F" => {
                    classify = true;
                }
                _ => paths.push(arg),
            }
        }

        if paths.is_empty() {
            paths.push(".");
        }

        for path_str in paths {
            let path = Path::new(path_str);
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap().to_string_lossy();
                    if !show_all && file_name.starts_with('.') {
                        continue;
                    }
                    let metadata = entry.metadata().unwrap();
                    let mut display_name = file_name.to_string();

                    if classify {
                        if metadata.is_dir() {
                            display_name.push('/');
                        } else if (metadata.permissions().mode() & 0o111) != 0 {
                            display_name.push('*');
                        } else if metadata.file_type().is_symlink() {
                            display_name.push('@');
                        }
                    }

                    if metadata.is_dir() {
                        display_name = display_name.blue().bold().to_string();
                    } else if (metadata.permissions().mode() & 0o111) != 0 {
                        display_name = display_name.green().bold().to_string();
                    }

                    if long {
                        let user = env::var("USER").unwrap_or("user".to_string());
                        let mode = metadata.permissions().mode();
                        let file_type = if metadata.is_dir() { 'd' } else { '-' };
                        let perms = [
                            ((mode & 0o400) != 0, 'r'),
                            ((mode & 0o200) != 0, 'w'),
                            ((mode & 0o100) != 0, 'x'),
                            ((mode & 0o040) != 0, 'r'),
                            ((mode & 0o020) != 0, 'w'),
                            ((mode & 0o010) != 0, 'x'),
                            ((mode & 0o004) != 0, 'r'),
                            ((mode & 0o002) != 0, 'w'),
                            ((mode & 0o001) != 0, 'x'),
                        ];

                        let perms_string: String = perms
                            .iter()
                            .map(|(set, c)| if *set { *c } else { '-' })
                            .collect();
                        println!("{}{} {} {}",file_type, &perms_string, user, display_name);
                    } else {
                        print!("{}  ", display_name);
                    }
                }
                if !long {
                    println!();
                }
            } else {
                eprintln!("ls: cannot access '{}': No such file or directory", path_str);
            }
        }
    }
}
