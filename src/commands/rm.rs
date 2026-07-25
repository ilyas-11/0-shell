
pub fn rm (args : &[&str]) {
     if args.is_empty() {
                    eprintln!("rm: missing operand");
                } else if args[0] == "-r" {
                    if args.len() < 2 {
                        eprintln!("rm: missing operand");
                    } else if let Err(err) = std::fs::remove_dir_all(args[1]) {
                        eprintln!("rm: {}", err);
                    }
                } else if let Err(err) = std::fs::remove_file(args[0]) {
                    eprintln!("rm: {}", err);
                }
}