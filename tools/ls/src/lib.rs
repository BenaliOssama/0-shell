use colored::*;
use std::{ fs };
use std::os::unix::fs::{ PermissionsExt, MetadataExt, FileTypeExt };
use std::path::{ Path, PathBuf };
use users::{ get_user_by_uid, get_group_by_gid };
use chrono::{ DateTime, Local };
use std::io::{Write,Read};
pub struct Ls;
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}
impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            stdin: Box::new(std::io::stdin()),
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &mut Cmd);
}

impl Command for Ls {
    fn name(&self) -> &'static str {
        "ls"
    }
    fn run(&self, cmd: &mut Cmd) {
        let mut show_all = false;
        let mut long = false;
        let mut classify = false;
        let mut paths: Vec<&str> = Vec::new();
        let mut all_paths: Vec<PathBuf> = Vec::new();

        // Parse args
        for arg in cmd.args.iter() {
            match arg.as_str() {
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

        // Collect paths
        for path_str in paths {
            let path = Path::new(path_str);

            if let Ok(entries) = fs::read_dir(path) {
                if show_all {
                    for special in [".", ".."] {
                        all_paths.push(PathBuf::from(special));
                    }
                }

                for entry in entries.flatten() {
                    let p = entry.path();
                    let file_name = p.file_name().unwrap().to_string_lossy();
                    if !show_all && file_name.starts_with('.') {
                        continue;
                    }
                    all_paths.push(p);
                }
            } else {
                cmd.stderr.write_all(format!("ls: cannot access '{}': No such file or directory\n", path_str).as_bytes()).unwrap();
            }
        }

        // Print total blocks if -l
        if long {
            let mut total_blocks = 0;
            for path in &all_paths {
                if let Ok(meta) = fs::metadata(path) {
                    total_blocks += meta.blocks();
                }
            }
            println!("total {}", total_blocks / 2);
        }
        all_paths.sort_by(|a, b| {
            let a_name = a
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| a.display().to_string());
            let b_name = b
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| b.display().to_string());
            a_name.cmp(&b_name)
        });

        display_entry(cmd, all_paths, long, classify);
    }
}

fn display_entry(cmd: &mut Cmd, paths: Vec<PathBuf>, long: bool, classify: bool) {
    for path in paths {
        if let Ok(metadata) = fs::symlink_metadata(&path) {
            // use symlink_metadata
            let file_name = path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());

            let mut display_name = file_name.clone();

            if classify {
                if metadata.is_dir() {
                    display_name.push('/');
                } else if metadata.file_type().is_symlink() {
                    display_name.push('@');
                } else if (metadata.permissions().mode() & 0o111) != 0 {
                    display_name.push('*');
                }
            }

            // Coloring
            if metadata.is_dir() {
                display_name = display_name.blue().bold().to_string();
            } else if
                metadata.file_type().is_char_device() ||
                metadata.file_type().is_block_device()
            {
                display_name = display_name.yellow().bold().to_string();
            }
            let mode = metadata.permissions().mode();
            let owner_full = (mode & 0o777) == 0o777;
            if owner_full && metadata.is_dir() {
                display_name = display_name.on_bright_green().to_string();
            }

            // Handle long format
            if long {
                let uid = metadata.uid();
                let gid = metadata.gid();

                let file_type = if metadata.is_dir() {
                    'd'
                } else if metadata.file_type().is_symlink() {
                    'l'
                } else if metadata.file_type().is_char_device() {
                    'c'
                } else if metadata.file_type().is_block_device() {
                    'b'
                } else {
                    '-'
                };

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

                // Symlink target
                if file_type == 'l' {
                    if let Ok(target) = fs::read_link(&path) {
                        display_name = format!(
                            "{} -> {}",
                            display_name.color(Color::BrightCyan).bold(),
                            target.display().to_string().yellow().bold()
                        );
                    }
                }

                let user_name = get_user_by_uid(uid)
                    .map(|u| u.name().to_string_lossy().into_owned())
                    .unwrap_or_else(|| uid.to_string());

                let group_name = get_group_by_gid(gid)
                    .map(|g| g.name().to_string_lossy().into_owned())
                    .unwrap_or_else(|| gid.to_string());
                let last_modified: DateTime<Local> = DateTime::<Local>::from(
                    metadata.modified().unwrap()
                );

                let (size_or_dev, _) = if file_type == 'c' || file_type == 'b' {
                    let rdev = metadata.rdev();
                    let major = (rdev >> 8) & 0xff;
                    let minor = rdev & 0xff;
                    (format!("{:>3}, {:>3}", major, minor), ())
                } else {
                    (format!("{:>8}", metadata.len()), ())
                };

                cmd.stdout.write_all(format!(
                    "{}{} {:<3} {:<8} {:<8} {} {} {}\n",
                    file_type,
                    perms_string,
                    metadata.nlink(),
                    user_name,
                    group_name,
                    size_or_dev,
                    last_modified.format("%b %e %H:%M"),
                    display_name
                ).as_bytes()).unwrap();
            } else {
                cmd.stdout.write_all(format!("{}  \n", display_name).as_bytes()).unwrap();
            }
        }
    }

    if !long {
        cmd.stdout.write_all(b"\n").unwrap();
    }
}
