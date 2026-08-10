use std::io::{self, Write};

mod commands;
mod helpers;

use helpers::parser::ParseError;
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

        if input.trim().is_empty() {
            continue;
        }

        let parts = loop {
            match helpers::parser::parse(input.trim_end()) {
                Ok(args) => break args,

                Err(ParseError::UnclosedDoubleQuote) => {
                    print!("dquote> ");
                    io::stdout().flush().unwrap();

                    let mut line = String::new();
                    stdin.read_line(&mut line).unwrap();

                    input.push_str(&line);
                }

                Err(ParseError::UnclosedSingleQuote) => {
                    print!("quote> ");
                    io::stdout().flush().unwrap();

                    let mut line = String::new();
                    stdin.read_line(&mut line).unwrap();

                    input.push_str(&line);
                }

                // Err(err) => {
                //     eprintln!("{:?}", err);
                //     continue;
                // }
            }
        };

        if parts.is_empty() {
            continue;
        }

        let parts: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();

        let command = parts[0];
        let args = &parts[1..];
        println!("command: {}, args: {:?}", command, args);

        match command {
            "exit" => commands::exit::exit(&args[1..]),
            "ls" => commands::ls::ls(args),
            "pwd" => commands::pwd::pwd(args),
            "cat" => commands::cat::cat(args),
            "cp" => commands::cp::cp(args),
            "mv" => commands::mv::mv(args),
            "rm" => commands::rm::rm(args),
            "mkdir" => commands::mkdir::mkdir(args),
            "echo" => commands::echo::echo(args),
            "cd" => commands::cd::cd(args),
            _ => println!("Command '{}' not found", command)
        }
    }
}