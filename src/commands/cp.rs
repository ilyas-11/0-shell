use std::path::{Path, PathBuf};
use crate::commands::{resolve_path};

pub fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("usage: cp <source> <destination>");
        return;
    }
    let source_path = resolve_path(args[0]);
    let destination_path = resolve_path(args[1]);

    let source = Path::new(&source_path);

    if source.is_dir() {
        eprintln!("cp: omitting directory '{}'", source.display());
        return;
    }
    
    let mut destination: PathBuf = PathBuf::from(destination_path);

    if destination.is_dir() {
        if let Some(file_name) = source.file_name() {
            destination.push(file_name);
        }
    }
    if source == destination {
        eprintln!("cp: '{}' and '{}' are the same file", source.display(), destination.display());
        return;
    }

    if let Err(err) = std::fs::copy(source, &destination) {
        eprintln!("cp: {}", err);
    }
}

//cp file.txt dir/file.txt
//cp folder dest
