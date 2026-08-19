use std::io::{self, Write};

mod commands;
mod helpers;

use helpers::parser::ParseError;

fn main() {
    let stdin = io::stdin();

    'shell: loop {
        print!("$ ");

        if let Err(err) = io::stdout().flush() {
            eprintln!("shell 1: {}", err);
            break;
        }

        let mut input = String::new();

        let bytes = match stdin.read_line(&mut input) {
            Ok(bytes) => bytes,
            Err(err) => {
                eprintln!("shell 2: {}", err);
                continue;
            }
        };

        if bytes == 0 {
            println!();
            break;
        }

        if input.trim().is_empty() {
            continue;
        }
        // let state = helpers::parser::parse(input.trim_end());
        // while state == incomplate {

        // }
        let parts = loop {
            match helpers::parser::parse(input.trim_end()) {
                Ok(args) => break args,
                Err(ParseError::LineContinuation) => {
                    print!("> ");

                    if let Err(err) = io::stdout().flush() {
                        eprintln!("shell 7: {}", err);
                        break 'shell;
                    }

                    let mut line = String::new();

                    match stdin.read_line(&mut line) {
                        Ok(0) => {
                            println!();
                            break 'shell;
                        }

                        Ok(_) => {
                            //println!("input: {}****", input);
                            input = input.trim_end_matches(['\n', '\r']).to_string();

                            if input.ends_with('\\') {
                                input.pop();
                            }

                            let line = line.trim_end_matches(['\n', '\r']);

                            input.push_str(line);
                        }

                        Err(err) => {
                            eprintln!("shell 8: {}", err);
                            break 'shell;
                        }
                    }
                }
                Err(ParseError::UnclosedDoubleQuote) => {
                    print!("dquote> ");

                    if let Err(err) = io::stdout().flush() {
                        eprintln!("shell 3: {}", err);
                        break 'shell;
                    }

                    let mut line = String::new();

                    match stdin.read_line(&mut line) {
                        Ok(0) => {
                            println!();
                            break 'shell;
                        }
                        Ok(_) => {
                            input.push_str(&line);
                        }
                        Err(err) => {
                            eprintln!("shell 4: {}", err);
                            break 'shell;
                        }
                    }
                }

                Err(ParseError::UnclosedSingleQuote) => {
                    print!("quote> ");

                    if let Err(err) = io::stdout().flush() {
                        eprintln!("shell 5: {}", err);
                        break 'shell;
                    }

                    let mut line = String::new();

                    match stdin.read_line(&mut line) {
                        Ok(0) => {
                            println!();
                            break 'shell;
                        }
                        Ok(_) => {
                            input.push_str(&line);
                        }
                        Err(err) => {
                            eprintln!("shell 6: {}", err);
                            break 'shell;
                        }
                    }
                }
            }
        };

        if parts.is_empty() {
            continue;
        }

        let parts: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" => commands::exit::exit(args),
            "ls" => commands::ls::ls(args),
            "pwd" => commands::pwd::pwd(args),
            "cat" => commands::cat::cat(args),
            "cp" => commands::cp::cp(args),
            "mv" => commands::mv::mv(args),
            "rm" => commands::rm::rm(args),
            "mkdir" => commands::mkdir::mkdir(args),
            "echo" => commands::echo::echo(args),
            "cd" => commands::cd::cd(args),
            _ => println!("Command '{}' not found", command),
        }
    }
}


//  cargo run | echo "jhjhjk"
