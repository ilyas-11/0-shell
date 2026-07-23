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

        // تجاهل السطر الفارغ
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
                println!("echo command");
            }

            "cd" => {
                println!("cd command");
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