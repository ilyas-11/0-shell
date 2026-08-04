pub fn pwd(args :&[&str]){
    if !args.is_empty() {
        eprintln!("pwd: too many arguments");
        return;
    }
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: {}", err),
    }
}