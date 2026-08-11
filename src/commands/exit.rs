use std::process;

pub fn exit(args: &[&str]) {
    println!("Exiting the shell...");
    if args.is_empty() {
        process::exit(0);
    }
    match args[0].parse::<i32>() {
        Ok(status) => {
            process::exit(status);
        }
        Err(_) => {
            eprintln!("exit: {}: numeric argument required", args[0]);
            process::exit(2);
        }
    }
}