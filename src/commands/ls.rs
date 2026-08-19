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


// ls -l Desktop -- Documents -a

// ls -l Desktop -- -a Documents 
// ls: cannot access '-a': No such file or directory
// Desktop:
// total 32
// drwxr-xr-x  9 iabid 10011 4096 Aug 10 12:40  0-shell
// drwxr-xr-x  7 iabid 10011 4096 Jul  2 16:06  Checkpoint-01
// drwxr-xr-x 11 iabid 10011 4096 Jul 18 14:32  Checkpoint-04-rust
// drwxr-xr-x 12 iabid 10011 4096 Jun 25 10:59 'New Folder 1'
// drwxr-xr-x 12 iabid 10011 4096 Jun 25 10:59 'New Folder 2'
// drwxr-xr-x 98 iabid 10011 4096 Jul 18 16:36  piscine-rust
// drwxr-xr-x  3 iabid 10011 4096 Jun 25 12:13  pool
// drwxr-xr-x  5 iabid 10011 4096 Aug 10 12:55  tt

// Documents:
// total 0
// iabid@z1r9s7:~$ 
