use std::path::*;
use crate::commands::{resolve_path};

pub fn mv(args : &[&str]) {
    if args.len() != 2 {
        eprintln!("usage: mv <source> <destination>");
        return;

    }  
    let source_path = resolve_path(args[0]);
    let destination_path = resolve_path(args[1]);

    
    let source = Path::new(&source_path);
    if !source.exists() {
        eprintln!(
            "mv: cannot stat '{}': No such file or directory",
            source.display()
        );
        return;
    }
    let mut destination = PathBuf::from(&destination_path);

    if destination.is_dir() {
        if let Some(name) = source.file_name() {
            destination.push(name);
        }
    }

    if let Err(err) = std::fs::rename(source, &destination) {
        eprintln!("mv: {}", err);
    }
}

