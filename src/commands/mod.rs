pub mod ls;
pub mod cat;
pub mod cp;
pub mod mv;
pub mod rm;
pub mod mkdir;
pub mod echo;
pub mod cd;
pub mod pwd;
pub mod exit;

use std::env;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn resolve_path(path: &str) -> String {
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());

    if path == "~" || path == "$HOME" {
        return home;
    }

    if let Some(rest) = path.strip_prefix("~/") {
        return format!("{}/{}", home, rest);
    }

    path.to_string()
}

/// `io::Error`'s `Display` appends ` (os error N)`; Unix tools do not print it.
pub fn err_msg(err: &io::Error) -> String {
    let msg = err.to_string();
    match msg.rfind(" (os error ") {
        Some(cut) if msg.ends_with(')') => msg[..cut].to_string(),
        _ => msg,
    }
}

/// An empty operand is shown as `''` in error messages, the way GNU tools do.
pub fn display_operand(operand: &str) -> String {
    if operand.is_empty() {
        "''".to_string()
    } else {
        operand.to_string()
    }
}

/// Two paths are the same file when they share a device and inode, so
/// `f.txt`, `./f.txt` and a hard link to it all compare equal.
pub fn is_same_file(a: &Path, b: &Path) -> bool {
    match (std::fs::metadata(a), std::fs::metadata(b)) {
        (Ok(a), Ok(b)) => a.dev() == b.dev() && a.ino() == b.ino(),
        _ => false,
    }
}
