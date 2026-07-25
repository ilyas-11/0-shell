pub fn cd(args: &[&str]) {
    if args.is_empty() {
                    eprintln!("cd: missing operand");
                } else if let Err(err) = std::env::set_current_dir(args[0]) {
                    eprintln!("cd: {}", err);
                }

}