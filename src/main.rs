use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        let bytes = stdin.read_line(&mut input).unwrap();

        if bytes == 0 {
            println!();
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        let command = parts[0];
        let args = &parts[1..];

        // println!("Command: {}", command);
        // println!("Arguments: {:?}", args);
        match command {
            "exit" => {
                break;
            }

            "pwd" => {
                match std::env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(err) => eprintln!("pwd: {}", err),
                }
            }

            "echo" => {
                println!("{}", args.join(" "));

            }

            "cd" => {
                if args.is_empty() {
                    eprintln!("cd: missing operand");
                } else if let Err(err) = std::env::set_current_dir(args[0]) {
                    eprintln!("cd: {}", err);
                }

            }

            "ls" => {
                println!("ls command");
            }

            _ => {
                println!("Command '{}' not found", command);
            }
        }
    }
}