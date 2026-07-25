pub fn cp(args : &[&str]) {
    if args.len() < 2 {
                    eprintln!("cp: missing operand");
                } else if let Err(err) = std::fs::copy(args[0], args[1]) {
                    eprintln!("cp: {}", err);
                }
}