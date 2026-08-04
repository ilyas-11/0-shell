use std::path::{Path, PathBuf};

pub fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("usage: cp <source> <destination>");
        return;
    }

    let source = Path::new(args[0]);

    if source.is_dir() {
        eprintln!("cp: omitting directory '{}'", source.display());
        return;
    }

    let mut destination: PathBuf = PathBuf::from(args[1]);

    if destination.is_dir() {
        if let Some(file_name) = source.file_name() {
            destination.push(file_name);
        }
    }

    if let Err(err) = std::fs::copy(source, &destination) {
        eprintln!("cp: {}", err);
    }
}

//cp file.txt dir/file.txt
//cp folder dest
