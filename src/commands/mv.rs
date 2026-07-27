use std::path::*;

pub fn mv(args : &[&str]) {
    if args.len() < 2 {
        eprintln!("mv: missing operand");

    }  
    let source = Path::new(&args[0]);
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