use std::io::{self, Write};
mod commands;
mod helpers;
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

        let parts: Vec<String> = helpers::parser::parse(input);
        let parts: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();

        let command = parts[0];
        let args = &parts[1..];
        match command {
            "exit" => break,
            "ls" => commands::ls::ls(args),
            "pwd" => commands::pwd::pwd(),
            "cat" => commands::cat::cat(args),
            "cp" => commands::cp::cp(args),
            "mv" => commands::mv::mv(args),
            "rm" => commands::rm::rm(args),
            "mkdir" => commands::mkdir::mkdir(args),
            "echo" => commands::echo::echo(args),
            "cd" => commands::cd::cd(args),
            _ => {
                println!("Command '{}' not found", command);
            }
        }
    }
}