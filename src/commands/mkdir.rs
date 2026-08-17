use crate::commands::{resolve_path};

pub fn mkdir (args :&[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
    }
    for file in args{
        let path= resolve_path(file);
        if let Err(err) = std::fs::create_dir(&path) {
            eprintln!("mkdir: {}: {}",file, err);
        }
    } 
}