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
 