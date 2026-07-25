pub fn mkdir (args :&[&str]) {
    if args.is_empty() {
                    eprintln!("mkdir: missing operand");
                } else if let Err(err) = std::fs::create_dir(args[0]) {
                    eprintln!("mkdir: {}", err);
                }
}