use std::path::*;
use crate::commands::{err_msg, is_same_file, resolve_path};

pub fn mv(args : &[&str]) {
    if args.len() != 2 {
        eprintln!("usage: mv <source> <destination>");
        return;

    }
    let source_path = resolve_path(args[0]);
    let destination_path = resolve_path(args[1]);


    let source = Path::new(&source_path);
    if source.symlink_metadata().is_err() {
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

    if is_same_file(source, &destination) {
        eprintln!("mv: '{}' and '{}' are the same file", source.display(), destination.display());
        return;
    }

    if let Err(err) = std::fs::rename(source, &destination) {
        eprintln!("mv: {}", err_msg(&err));
    }
}
// mv a.txt b.txt q  hi

