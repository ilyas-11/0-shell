pub fn mkdir (args :&[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
    }
    for file in args{
        if let Err(err) = std::fs::create_dir(file) {
            eprintln!("mkdir: {}: {}",file, err);
        }
    } 
}