pub fn cd(args: &[&str]) {
   let target = if args.is_empty() {
       std::env::var("HOME")
       .or_else(|_| std::env::var("USERPROFILE"))
       .unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].to_string()
    };
    if args.len()>1 {
       eprintln!("cd: too many arguments");
       return;
   }
    if let Err(err) = std::env::set_current_dir(&target) {
        eprintln!("cd: {}", err);
    }

}


// cd /Desktop
// cd hi hi
//  cd -- cd cd ~ cd $Home