// use std::fs;

// use std::os::unix::fs::PermissionsExt;

// fn permissions_to_string(mode: u32) -> String {
//     let mut perms = String::new();

//     let flags = [
//         (0o400, 'r'),
//         (0o200, 'w'),
//         (0o100, 'x'),
//         (0o040, 'r'),
//         (0o020, 'w'),
//         (0o010, 'x'),
//         (0o004, 'r'),
//         (0o002, 'w'),
//         (0o001, 'x'),
//     ];

//     for (flag, ch) in flags {
//         if mode & flag != 0 {
//             perms.push(ch);
//         } else {
//             perms.push('-');
//         }
//     }

//     perms
// }

// pub fn ls(args: &[&str]) {
//     let show_hidden = args.contains(&"-a");
//     let long_format = args.contains(&"-l");
//     let classify = args.contains(&"-F");

//     let entries = match fs::read_dir(".") {
//         Ok(entries) => entries,
//         Err(err) => {
//             eprintln!("ls: {}", err);
//             return;
//         }
//     };

//     for entry in entries {
//         let entry = match entry {
//             Ok(e) => e,
//             Err(err) => {
//                 eprintln!("ls: {}", err);
//                 continue;
//             }
//         };

//         let name = entry.file_name();
//         let name = name.to_string_lossy();

//         // إخفاء الملفات التي تبدأ بـ '.'
//         if !show_hidden && name.starts_with('.') {
//             continue;
//         }

//         let metadata = match entry.metadata() {
//             Ok(m) => m,
//             Err(err) => {
//                 eprintln!("ls: {}", err);
//                 continue;
//             }
//         };

//         let mut display_name = name.to_string();

//         if classify && metadata.is_dir() {
//             display_name.push('/');
//         }

//         if long_format {
//             let file_type = if metadata.is_dir() { 'd' } else { '-' };

//             let permissions =
//                 permissions_to_string(metadata.permissions().mode());

//             println!(
//                 "{}{} {:>10} {}",
//                 file_type,
//                 permissions,
//                 metadata.len(),
//                 display_name
//             );
//         } else {
//             println!("{}", display_name);
//         }
//     }
// }