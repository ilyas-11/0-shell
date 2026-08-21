
use crate::commands::{err_msg, resolve_path};
pub fn cd(args: &[&str], old_pwd: &mut Option<String>) {
    if args.len() > 2 {
        eprintln!("cd: too many arguments");
        return;
    }

    let target = if args.is_empty() {
        match std::env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME not set");
                return;
            }
        }
    } else if args[0] == "--" {
        if args.len() == 1 {
            match std::env::var("HOME") {
                Ok(home) => home,
                Err(_) => {
                    eprintln!("cd: HOME not set");
                    return;
                }
            }
        } else {
            resolve_path(args[1])
        }
    } else if args[0] == "-" {
        match old_pwd {
            Some(path) => path.clone(),
            None => {
                eprintln!("cd: OLDPWD not set");
                return;
            }
        }
    } else {
        resolve_path(args[0])
    };
    let current = match std::env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("cd: {}", err_msg(&err));
            return;
        }
    };

    // `cd ""` stays where it is, but still records OLDPWD, like sh and bash.
    if target.is_empty() {
        *old_pwd = Some(current.to_string_lossy().to_string());
        return;
    }

    if let Err(err) = std::env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, err_msg(&err));
    } else {
        *old_pwd = Some(current.to_string_lossy().to_string());
    }
    if args.first() == Some(&"-") {
        println!("{}", target);
    }
}


// cd ~ => /home/iabid
// cd "$HOME" => /home/iabid
// cd "" => no chenge
//  cd  cd cd ~ cd $Home