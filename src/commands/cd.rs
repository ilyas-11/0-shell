
use crate::commands::{resolve_path};
pub fn cd(args: &[&str]) {
    if args.len() > 1 {
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
    } else {
        resolve_path(args[0])
    };
    println!("cd: Changing directory to {}", target);
    if let Err(err) = std::env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, err);
    }
}


// cd ~ => /home/iabid
// cd "$HOME" => /home/iabid
// cd "" => no chenge
//  cd  cd cd ~ cd $Home