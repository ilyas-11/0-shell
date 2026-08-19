use std::process;

pub fn exit(args: &[&str]) {
    if args.is_empty() {
        process::exit(0);
    }
    if args[0].chars().any(|c| !c.is_digit(10)) {
        eprintln!("exit: {}: numeric argument required", args[0]);
        return;
    }
    
    match args[0].parse::<i128>() {
        Ok(status) => {
            if status < 0 {
                eprintln!("exit: {}: numeric argument required", args[0]);
                return;
            }
            process::exit((status%256) as i32);

        }
        Err(_) => {
            eprintln!("exit: {}: numeric argument required", args[0]);
            process::exit(255);
        }
    }
}