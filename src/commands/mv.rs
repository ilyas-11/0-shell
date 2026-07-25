pub fn mv(args : &[&str]) {
    if args.len() < 2 {
                    eprintln!("mv: missing operand");
                } else if let Err(err) = std::fs::rename(args[0], args[1]) {
                    eprintln!("mv: {}", err);
                }
}