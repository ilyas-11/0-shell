use std::path::{Path, PathBuf};
use crate::commands::{err_msg, is_same_file, resolve_path};

pub fn mv(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mv: missing file operand");
        return;
    }
    if args.len() == 1 {
        eprintln!("mv: missing destination file operand after '{}'", args[0]);
        return;
    }

    let (sources, target) = args.split_at(args.len() - 1);
    let target_path = resolve_path(target[0]);
    let target = Path::new(&target_path);

    // Several sources have nowhere to land but a directory, so refuse the whole
    // list up front instead of moving part of it and failing on the rest.
    if sources.len() > 1 && !target.is_dir() {
        if target.symlink_metadata().is_ok() {
            eprintln!("mv: target '{}': Not a directory", target.display());
        } else {
            eprintln!("mv: target '{}': No such file or directory", target.display());
        }
        return;
    }

    for source in sources {
        move_one(source, target);
    }
}

/// Moves one operand. A failure is reported and the remaining operands are still
/// attempted, so one bad name in a long list does not hide the rest.
fn move_one(source_arg: &str, target: &Path) {
    let source_path = resolve_path(source_arg);
    let source = Path::new(&source_path);

    if source.symlink_metadata().is_err() {
        eprintln!(
            "mv: cannot stat '{}': No such file or directory",
            source.display()
        );
        return;
    }

    let mut destination = PathBuf::from(target);

    if destination.is_dir() {
        if let Some(name) = source.file_name() {
            destination.push(name);
        }
    }

    if is_same_file(source, &destination) {
        eprintln!(
            "mv: '{}' and '{}' are the same file",
            source.display(),
            destination.display()
        );
        return;
    }

    if let Err(err) = std::fs::rename(source, &destination) {
        eprintln!(
            "mv: cannot move '{}' to '{}': {}",
            source.display(),
            destination.display(),
            err_msg(&err)
        );
    }
}
// mv a.txt b.txt q  hi
