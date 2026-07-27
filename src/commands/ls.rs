use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
fn permissions_to_string(mode: u32) -> String {
    let mut perms = String::new();

    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for (flag, ch) in flags {
        if mode & flag != 0 {
            perms.push(ch);
        } else {
            perms.push('-');
        }
    }

    perms
}

pub fn ls(args: &[&str]) {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut classify = false;
    let mut path = ".";
    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_hidden = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => {}
                }
            }
        } else {
            path = arg;
        }
    }
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("ls: {}", err);
            return;
        }
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !show_hidden && name.starts_with('.') {
            continue;
        }
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(err) => {
                eprintln!("ls: {}", err);
                continue;
            }
        };
        let mut display_name = name.to_string();
        if classify && metadata.is_dir() {
            display_name.push('/');
        }
        if long_format {
            let file_type = if metadata.is_dir() { 'd' } else { '-' };

            #[cfg(unix)]
            let permissions = permissions_to_string(metadata.permissions().mode());

            #[cfg(not(unix))]
            let permissions = "---------".to_string();

            println!(
                "{}{} {:>10} {}",
                file_type,
                permissions,
                metadata.len(),
                display_name
            );
        } else {
            println!("{}", display_name);
        }
    }
}