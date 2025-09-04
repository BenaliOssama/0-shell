use chrono::{DateTime, Local};
use colored::*;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

pub struct Ls;

pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    _stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            _stdin: Box::new(std::io::stdin()),
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
        let args_clone = cmd.args.clone();
        for arg in args_clone.iter() {
            match arg.as_str() {
                "-a" => show_all = true,
                "-l" => long = true,
                "-F" => classify = true,
                _ => paths.push(arg),
            }
        }

        if paths.is_empty() {
            paths.push(".");
        }

        let multiple_dirs = paths.len() > 1;

        for (i, path_str) in paths.iter().enumerate() {
            let path = Path::new(path_str);

            if multiple_dirs {
                if i > 0 {
                    let _ = writeln!(cmd.stdout);
                }
                if path.is_dir() {
                    let _ = writeln!(cmd.stdout, "{}:", path.display());
                }
            }

            if path.is_file() || path.is_symlink() {
                display_entry(cmd, vec![path.to_path_buf()], long, classify);
                continue;
            }

            let read_dir = fs::read_dir(path);
            if read_dir.is_err() {
                let _ = writeln!(
                    cmd.stderr,
                    "ls: cannot access '{}': No such file or directory",
                    path.display()
                );
                continue;
            }

            let mut entries: Vec<PathBuf> = Vec::new();

            if show_all {
                entries.push(path.join("."));
                entries.push(path.join(".."));
            }

            for entry in read_dir.unwrap().flatten() {
                let p = entry.path();
                if !show_all {
                    if let Some(name) = p.file_name() {
                        if name.to_string_lossy().starts_with('.') {
                            continue;
                        }
                    }
                }
                entries.push(p);
            }

            entries.sort_by(|a, b| {
                let a_name = a
                    .file_name()
                    .map(|f| f.to_string_lossy().into_owned())
                    .unwrap_or_default()
                    .to_lowercase();
                let b_name = b
                    .file_name()
                    .map(|f| f.to_string_lossy().into_owned())
                    .unwrap_or_default()
                    .to_lowercase();
                a_name.cmp(&b_name)
            });

            if long {
                let mut total_blocks = 0;
                for entry in &entries {
                    if let Ok(meta) = fs::symlink_metadata(entry) {
                        total_blocks += meta.blocks();
                    }
                }
                let _ = writeln!(cmd.stdout, "total {}", total_blocks / 2);
            }

            display_entry(cmd, entries, long, classify);
        }
    }
}

fn display_entry(cmd: &mut Cmd, paths: Vec<PathBuf>, long: bool, classify: bool) {
    // First, collect metadata and display info for each path
    struct Entry {
        path: PathBuf,
        metadata: fs::Metadata,
        user_name: String,
        group_name: String,
        size_or_dev: String,
        file_type_char: char,
        perms_string: String,
        last_modified: DateTime<Local>,
        display_name: String,
        symlink_target: String,
        nlink: u64,
    }

    let mut entries = vec![];

    for path in paths.iter() {
        if let Ok(metadata) = fs::symlink_metadata(path) {
            let file_name = path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());

            let mut display_name = file_name.clone();

            if classify {
                let ftype = metadata.file_type();
                if ftype.is_dir() {
                    display_name.push('/');
                } else if ftype.is_symlink() {
                    display_name.push('@');
                } else if ftype.is_socket() {
                    display_name.push('=');
                } else if ftype.is_fifo() {
                    display_name.push('|');
                } else if (metadata.permissions().mode() & 0o111) != 0 {
                    display_name.push('*');
                }
            }

            if metadata.is_dir() {
                display_name = display_name.blue().bold().to_string();
            } else if metadata.file_type().is_symlink() {
                display_name = display_name.cyan().bold().to_string();
            } else if metadata.file_type().is_socket() {
                display_name = display_name.magenta().to_string();
            } else if metadata.file_type().is_fifo() {
                display_name = display_name.yellow().to_string();
            } else if (metadata.permissions().mode() & 0o111) != 0 {
                display_name = display_name.green().to_string();
            }

            let uid = metadata.uid();
            let gid = metadata.gid();

            let file_type_char = match metadata.file_type() {
                ft if ft.is_dir() => 'd',
                ft if ft.is_symlink() => 'l',
                ft if ft.is_char_device() => 'c',
                ft if ft.is_block_device() => 'b',
                ft if ft.is_fifo() => 'p',
                ft if ft.is_socket() => 's',
                _ => '-',
            };

            let perms_string = format_mode(metadata.permissions().mode());

            let symlink_target = if file_type_char == 'l' {
                fs::read_link(path)
                    .map(|t| format!(" -> {}", t.display()))
                    .unwrap_or_default()
            } else {
                String::new()
            };

            let user_name = get_user_by_uid(uid)
                .map(|u| u.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| uid.to_string());

            let group_name = get_group_by_gid(gid)
                .map(|g| g.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| gid.to_string());

            let last_modified: DateTime<Local> = DateTime::<Local>::from(
                metadata
                    .modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now()),
            );

            let size_or_dev = if file_type_char == 'c' || file_type_char == 'b' {
                let rdev = metadata.rdev();
                format!("{:>3}, {:>3}", major(rdev), minor(rdev))
            } else {
                metadata.len().to_string()
            };

            entries.push(Entry {
                path: path.clone(),
                metadata: metadata.clone(),
                user_name,
                group_name,
                size_or_dev,
                file_type_char,
                perms_string,
                last_modified,
                display_name,
                symlink_target,
                nlink: metadata.nlink(),
            });
        }
    }

    // Compute max widths
    let max_link_width = entries
        .iter()
        .map(|e| e.nlink.to_string().len())
        .max()
        .unwrap_or(1);
    let max_user_width = entries.iter().map(|e| e.user_name.len()).max().unwrap_or(1);
    let max_group_width = entries
        .iter()
        .map(|e| e.group_name.len())
        .max()
        .unwrap_or(1);
    let max_size_width = entries
        .iter()
        .map(|e| e.size_or_dev.len())
        .max()
        .unwrap_or(1);

    // Print entries
    for entry in entries {
        if long {
            let _ = writeln!(
                cmd.stdout,
                "{}{} {:>width_link$} {:<width_user$} {:<width_group$} {:>width_size$} {} {}{}",
                entry.file_type_char,
                entry.perms_string,
                entry.nlink,
                entry.user_name,
                entry.group_name,
                entry.size_or_dev,
                entry.last_modified.format("%b %e %H:%M"),
                entry.display_name,
                entry.symlink_target,
                width_link = max_link_width,
                width_user = max_user_width,
                width_group = max_group_width,
                width_size = max_size_width
            );
        } else {
            let _ = writeln!(cmd.stdout, "{}", entry.display_name);
        }
    }
}

fn format_mode(mode: u32) -> String {
    let usr = (mode >> 6) & 0o7;
    let grp = (mode >> 3) & 0o7;
    let oth = mode & 0o7;

    let mut perms = String::with_capacity(9);

    perms.push(if usr & 0o4 != 0 { 'r' } else { '-' });
    perms.push(if usr & 0o2 != 0 { 'w' } else { '-' });
    perms.push(if usr & 0o1 != 0 { 'x' } else { '-' });

    perms.push(if grp & 0o4 != 0 { 'r' } else { '-' });
    perms.push(if grp & 0o2 != 0 { 'w' } else { '-' });
    perms.push(if grp & 0o1 != 0 { 'x' } else { '-' });

    perms.push(if oth & 0o4 != 0 { 'r' } else { '-' });
    perms.push(if oth & 0o2 != 0 { 'w' } else { '-' });
    perms.push(if oth & 0o1 != 0 { 'x' } else { '-' });

    // Handle special permission bits
    // setuid (0o4000), setgid (0o2000), sticky (0o1000)
    let suid = (mode & 0o4000) != 0;
    let sgid = (mode & 0o2000) != 0;
    let sticky = (mode & 0o1000) != 0;

    if suid {
        perms.replace_range(
            2..3,
            if perms.as_bytes()[2] == b'x' {
                "s"
            } else {
                "S"
            },
        );
    }
    if sgid {
        perms.replace_range(
            5..6,
            if perms.as_bytes()[5] == b'x' {
                "s"
            } else {
                "S"
            },
        );
    }
    if sticky {
        perms.replace_range(
            8..9,
            if perms.as_bytes()[8] == b'x' {
                "t"
            } else {
                "T"
            },
        );
    }

    perms
}

fn major(dev: u64) -> u64 {
    (dev >> 8) & 0xfff
}

fn minor(dev: u64) -> u64 {
    (dev & 0xff) | ((dev >> 12) & 0xfff00)
}