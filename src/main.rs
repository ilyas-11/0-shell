use std::io::{self, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};

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
            "mkdir" => {
               if args.is_empty() {
                    eprintln!("mkdir: missing operand");
                } else if let Err(err) = std::fs::create_dir(args[0]) {
                    eprintln!("mkdir: {}", err);
                }
            }

            "ls" => {
                let show_hidden = args.contains(&"-a");
                let classify = args.contains(&"-F");
                match std::fs::read_dir(".") {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(entry) => {
                                    let name = entry.file_name();
                                    let name = name.to_string_lossy();

                                    if !show_hidden && name.starts_with('.') {
                                        continue;
                                    }

                                    match entry.file_type() {
                                        Ok(file_type) => {
                                            if classify && file_type.is_dir() {
                                                println!("{}/", name);
                                            } else {
                                                println!("{}", name);
                                            }
                                        }
                                        Err(err) => {
                                            eprintln!("ls: {}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("ls: {}", err);
                                }
                            }
                        }
                    }

                    Err(err) => {
                        eprintln!("ls: {}", err);
                    }
                }
            }

            _ => {
                println!("Command '{}' not found", command);
            }
        }
    }
}