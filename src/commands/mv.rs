use std::path::*;

pub fn mv(args : &[&str]) {
    if args.len() != 2 {
        eprintln!("usage: mv <source> <destination>");
        return;

    }  
    let source = Path::new(&args[0]);
    if !source.exists() {
        eprintln!(
            "mv: cannot stat '{}': No such file or directory",
            source.display()
        );
        return;
    }
    let mut destination = PathBuf::from(&args[1]);

    if destination.is_dir() {
        if let Some(name) = source.file_name() {
            destination.push(name);
        }
    }

    if let Err(err) = std::fs::rename(source, &destination) {
        eprintln!("mv: {}", err);
    }
}
// mv a.txt b.txt q  hi

