use crate::commands::{resolve_path};

pub fn rm (args : &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    if args[0] == "-r" {
        let path = resolve_path(args[1]);
        if args.len() < 2 {
            eprintln!("rm: missing operand");
        } else if let Err(err) = std::fs::remove_dir_all(&path) {
            eprintln!("rm: {}", err);
        }
    } else if let Err(err) = std::fs::remove_file(args[0]) {
                    eprintln!("rm: {}", err);
    }
}