use crate::commands::{err_msg, resolve_path};
use std::path::Path;

/// True when the last component of `path` is `.` or `..`. Removing such a
/// directory recursively would delete its contents before the syscall failed,
/// so it has to be refused up front.
fn is_dot_dir(path: &str) -> bool {
    let trimmed = path.trim_end_matches('/');
    let last = trimmed.rsplit('/').next().unwrap_or("");
    last == "." || last == ".."
}

/// True when `path` is the root directory (`/`, `//`, ...).
fn is_root(path: &str) -> bool {
    path.starts_with('/') && path.trim_start_matches('/').is_empty()
}

pub fn rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    let mut recursive = false;
    let mut skip_flag = false;
    let mut paths = Vec::new();
    
    for arg in args {
        if skip_flag {
            paths.push(*arg);
            continue;
        }
        
        if *arg == "--" {
            skip_flag = true;
            continue;
        }
        
        if arg.starts_with('-') && *arg != "-" {
            if *arg == "-r" || *arg == "-R" || *arg == "--recursive" {
                recursive = true;
            } else {
                let mut is_r = false;
                let mut invalid_char = None;
                for ch in arg.chars().skip(1) {
                    if ch == 'r' || ch == 'R' {
                        is_r = true;
                    } else if ch == 'f' || ch == 'i' || ch == 'v' {
                        // safely ignore standard optional flags
                    } else {
                        invalid_char = Some(ch);
                        break;
                    }
                }
                
                if let Some(ch) = invalid_char {
                    eprintln!("rm: invalid option -- '{}'", ch);
                    return;
                }
                if is_r {
                    recursive = true;
                }
            }
        } else {
            paths.push(*arg);
        }
    }
    
    if paths.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    
    for path in paths {
        let resolved_path_str = resolve_path(path);
        let path_obj = Path::new(&resolved_path_str);
        
        let meta = match std::fs::symlink_metadata(path_obj) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", path, err_msg(&e));
                continue;
            }
        };

        let result = if meta.is_dir() {
            if recursive {
                if is_dot_dir(path) {
                    eprintln!("rm: refusing to remove '.' or '..' directory: skipping '{}'", path);
                    continue;
                }
                if is_root(&resolved_path_str) {
                    eprintln!("rm: it is dangerous to operate recursively on '{}'", path);
                    eprintln!("rm: use --no-preserve-root to override this failsafe");
                    continue;
                }
                std::fs::remove_dir_all(path_obj)
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path);
                continue;
            }
        } else {
            std::fs::remove_file(path_obj)
        };

        if let Err(err) = result {
            eprintln!("rm: cannot remove '{}': {}", path, err_msg(&err));
        }
    }
}
