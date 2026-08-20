use std::process;

pub fn exit(args: &[&str]) {
    if args.is_empty() {
        process::exit(0);
    }
    
    match args[0].parse::<i128>() {
        Ok(status) => {
            if args.len() > 1 {
                eprintln!("exit: too many arguments");
                // Bash does not exit if the first argument is numeric but there are too many arguments
                return;
            }
            let exit_code = ((status % 256) + 256) % 256;
            process::exit(exit_code as i32);
        }
        Err(_) => {
            eprintln!("exit: {}: numeric argument required", args[0]);
            process::exit(255);
        }
    }
}