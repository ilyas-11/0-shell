use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::cmp::Ordering;

#[repr(C)]
struct tm {
    tm_sec: i32, tm_min: i32, tm_hour: i32, tm_mday: i32, tm_mon: i32,
    tm_year: i32, tm_wday: i32, tm_yday: i32, tm_isdst: i32, tm_gmtoff: i64, tm_zone: *const i8,
}
unsafe extern "C" { fn localtime_r(timep: *const i64, result: *mut tm) -> *mut tm; }

fn format_mtime(mtime: i64) -> String {
    let mut tm = tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 0, tm_mon: 0, tm_year: 0, tm_wday: 0, tm_yday: 0, tm_isdst: 0, tm_gmtoff: 0, tm_zone: std::ptr::null() };
    unsafe { localtime_r(&mtime, &mut tm); }
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let month = months[(tm.tm_mon as usize) % 12];
    format!("{} {:2} {:02}:{:02}", month, tm.tm_mday, tm.tm_hour, tm.tm_min)
}

struct UsersGroups {
    users: HashMap<u32, String>,
    groups: HashMap<u32, String>,
}

impl UsersGroups {
    fn new() -> Self {
        let mut users = HashMap::new();
        let mut groups = HashMap::new();

        if let Ok(content) = fs::read_to_string("/etc/passwd") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    if let Ok(uid) = parts[2].parse::<u32>() {
                        users.insert(uid, parts[0].to_string());
                    }
                }
            }
        }
        if let Ok(content) = fs::read_to_string("/etc/group") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    if let Ok(gid) = parts[2].parse::<u32>() {
                        groups.insert(gid, parts[0].to_string());
                    }
                }
            }
        }

        Self { users, groups }
    }

    fn get_user(&self, uid: u32) -> String {
        self.users.get(&uid).cloned().unwrap_or_else(|| uid.to_string())
    }

    fn get_group(&self, gid: u32) -> String {
        self.groups.get(&gid).cloned().unwrap_or_else(|| gid.to_string())
    }
}

fn file_type_char(ft: fs::FileType) -> char {
    if ft.is_dir() { 'd' }
    else if ft.is_symlink() { 'l' }
    else if ft.is_char_device() { 'c' }
    else if ft.is_block_device() { 'b' }
    else if ft.is_fifo() { 'p' }
    else if ft.is_socket() { 's' }
    else { '-' }
}

fn permissions_string(mode: u32) -> String {
    let mut s = String::with_capacity(9);
    let rwx = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];
    s.push_str(rwx[((mode >> 6) & 7) as usize]);
    s.push_str(rwx[((mode >> 3) & 7) as usize]);
    s.push_str(rwx[(mode & 7) as usize]);
    
    let mut chars: Vec<char> = s.chars().collect();
    if mode & 0o4000 != 0 { chars[2] = if mode & 0o100 != 0 { 's' } else { 'S' }; }
    if mode & 0o2000 != 0 { chars[5] = if mode & 0o010 != 0 { 's' } else { 'S' }; }
    if mode & 0o1000 != 0 { chars[8] = if mode & 0o001 != 0 { 't' } else { 'T' }; }
    chars.into_iter().collect()
}

fn classify_char(meta: &fs::Metadata) -> Option<char> {
    let ft = meta.file_type();
    let mode = meta.mode();
    if ft.is_dir() {
        Some('/')
    } else if ft.is_symlink() {
        Some('@')
    } else if ft.is_socket() {
        Some('=')
    } else if ft.is_fifo() {
        Some('|')
    } else if mode & 0o111 != 0 {
        Some('*')
    } else {
        None
    }
}

fn get_symlink_target(path: &Path) -> String {
    if let Ok(target) = fs::read_link(path) {
        format!(" -> {}", target.display())
    } else {
        String::new()
    }
}

struct EntryInfo {
    name: String,
    path: PathBuf,
    meta: fs::Metadata,
}

impl EntryInfo {
    fn new(name: String, path: PathBuf, meta: fs::Metadata) -> Self {
        Self { name, path, meta }
    }
}

fn print_entries(entries: &[EntryInfo], long_format: bool, classify: bool, ug: &UsersGroups, is_dir_contents: bool) {
    if entries.is_empty() {
        return;
    }
    
    if long_format {
        let mut max_links = 0;
        let mut max_user = 0;
        let mut max_group = 0;
        let mut max_size = 0;
        let mut total_blocks = 0;

        for e in entries {
            max_links = max_links.max(e.meta.nlink().to_string().len());
            max_user = max_user.max(ug.get_user(e.meta.uid()).len());
            max_group = max_group.max(ug.get_group(e.meta.gid()).len());
            max_size = max_size.max(e.meta.len().to_string().len());
            total_blocks += e.meta.blocks();
        }

        if is_dir_contents {
            println!("total {}", total_blocks / 2);
        }

        for e in entries {
            let ft = file_type_char(e.meta.file_type());
            let perms = permissions_string(e.meta.mode());
            let links = e.meta.nlink().to_string();
            let user = ug.get_user(e.meta.uid());
            let group = ug.get_group(e.meta.gid());
            let size = e.meta.len().to_string();
            let mtime = format_mtime(e.meta.mtime());
            
            let mut name = e.name.clone();
            if classify {
                if let Some(c) = classify_char(&e.meta) {
                    name.push(c);
                }
            }
            
            let symlink = if e.meta.file_type().is_symlink() {
                get_symlink_target(&e.path)
            } else {
                String::new()
            };
            
            println!("{}{} {:>links_w$} {:<user_w$} {:<group_w$} {:>size_w$} {} {}{}",
                ft, perms, links, user, group, size, mtime, name, symlink,
                links_w = max_links, user_w = max_user, group_w = max_group, size_w = max_size
            );
        }
    } else {
        for e in entries {
            let mut name = e.name.clone();
            if classify {
                if let Some(c) = classify_char(&e.meta) {
                    name.push(c);
                }
            }
            println!("{}", name);
        }
    }
}

fn ls_cmp(a: &str, b: &str) -> Ordering {
    if a == b { return Ordering::Equal; }
    if a == "." { return Ordering::Less; }
    if b == "." { return Ordering::Greater; }
    if a == ".." { return Ordering::Less; }
    if b == ".." { return Ordering::Greater; }
    
    let a_clean: String = a.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    let b_clean: String = b.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    
    let cmp = a_clean.cmp(&b_clean);
    if cmp == Ordering::Equal {
        a.cmp(b)
    } else {
        cmp
    }
}

pub fn ls(args: &[&str]) {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut classify = false;
    let mut paths = Vec::new();

    let mut parse_flags = true;
    for arg in args {
        if parse_flags && arg.starts_with('-') && *arg != "-" {
            if *arg == "--" {
                parse_flags = false;
                continue;
            }
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_hidden = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => {
                        eprintln!("ls: invalid option -- '{}'", ch);
                        return;
                    }
                }
            }
        } else {
            paths.push(arg.to_string());
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }
    
    let ug = UsersGroups::new();
    
    let mut errs = Vec::new();
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    
    for path in &paths {
        let meta_res = if long_format {
            fs::symlink_metadata(path)
        } else {
            fs::metadata(path).or_else(|_| fs::symlink_metadata(path))
        };
        
        match meta_res {
            Ok(meta) => {
                if meta.is_dir() {
                    dirs.push(path.clone());
                } else {
                    files.push(EntryInfo::new(path.clone(), PathBuf::from(path), meta));
                }
            }
            Err(e) => {
                errs.push((path.clone(), e));
            }
        }
    }
    
    errs.sort_by(|a, b| a.0.cmp(&b.0));
    files.sort_by(|a, b| ls_cmp(&a.name, &b.name));
    dirs.sort();
    
    for (path, err) in &errs {
        eprintln!("ls: cannot access '{}': {}", path, err);
    }
    
    let multiple_targets = paths.len() > 1;
    let mut first = errs.is_empty();
    
    if !files.is_empty() {
        print_entries(&files, long_format, classify, &ug, false);
        first = false;
    }
    
    for dir in dirs {
        if !first {
            println!();
        }
        first = false;
        
        if multiple_targets {
            println!("{}:", dir);
        }
        
        let mut entries = Vec::new();
        match fs::read_dir(&dir) {
            Ok(iter) => {
                if show_hidden {
                    let dir_path = Path::new(&dir);
                    let dot = dir_path.join(".");
                    if let Ok(meta) = fs::symlink_metadata(&dot) {
                        entries.push(EntryInfo::new(".".to_string(), dot, meta));
                    }
                    let dotdot = dir_path.join("..");
                    if let Ok(meta) = fs::symlink_metadata(&dotdot) {
                        entries.push(EntryInfo::new("..".to_string(), dotdot, meta));
                    }
                }
                
                for entry_res in iter {
                    if let Ok(entry) = entry_res {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        if !show_hidden && name.starts_with('.') {
                            continue;
                        }
                        if let Ok(meta) = entry.path().symlink_metadata() {
                            entries.push(EntryInfo::new(name, entry.path(), meta));
                        }
                    }
                }
                
                entries.sort_by(|a, b| ls_cmp(&a.name, &b.name));
                print_entries(&entries, long_format, classify, &ug, true);
            }
            Err(e) => {
                eprintln!("ls: cannot open directory '{}': {}", dir, e);
            }
        }
    }
}
