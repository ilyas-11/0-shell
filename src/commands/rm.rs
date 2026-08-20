use crate::commands::resolve_path;
use std::path::Path;

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
                eprintln!("rm: cannot remove '{}': {}", path, e);
                continue;
            }
        };

        let result = if meta.is_dir() {
            if recursive {
                std::fs::remove_dir_all(path_obj)
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path);
                continue;
            }
        } else {
            std::fs::remove_file(path_obj)
        };

        if let Err(err) = result {
            eprintln!("rm: cannot remove '{}': {}", path, err);
        }
    }
}