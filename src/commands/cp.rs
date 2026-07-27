use std::path::Path;

pub fn cp(args : &[&str]) {
    eprintln!("{:?}",args);
    if args.len() < 2 {
        eprintln!("cp: missing operand");
        return;
    }
    let source = Path::new(&args[0]);
    let mut destination = Path::new(&args[1]).to_path_buf();

    if destination.is_dir() {
        if let Some(file_name) = source.file_name() {
            destination.push(file_name);
        }
    }
    if let Err(err) = std::fs::copy(source, destination) {
        eprintln!("**cp: {}", err);
    }
}