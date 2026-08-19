use crate::commands::{resolve_path};
use std::path::Path;

pub fn rm (args : &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    let mut recursive = false;
    let mut skip_flag = false;
    let mut paths = Vec::new();
    for arg in args {
        if *arg== "--"{
            skip_flag = true;
            continue;
        }
        if !skip_flag && (*arg == "-r"|| *arg == "-R"||*arg == "--recursive"){
            recursive = true;
        } else {
            paths.push(*arg);
        }
    }
    if paths.is_empty(){
        eprintln!("rm: missing operand");
        return;
    }
    for path in paths {
        let path = resolve_path(path);
        let path_obj = Path::new(&path);

        let result = if path_obj.is_dir() {
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
            eprintln!("rm: {}: {}", path, err);
        }
    }
}