/// ascii-escape colors

pub const RED: &str  = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";

pub const BOLD: &str = "\x1b[1m";

pub const RESET: &str = "\x1b[0m";

pub fn red(text: &str) -> String {
    format!("{RED}{text}{RESET}")
}

pub fn green(text: &str) -> String {
    format!("{GREEN}{text}{RESET}")
}
