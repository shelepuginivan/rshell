/// ascii-escape colors

pub const RED: &str  = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const BOLD: &str = "\x1b[1m";
pub const DIMMED: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[1m";

pub const RESET: &str = "\x1b[0m";

pub fn red(text: &str) -> String {
    format!("{RED}{text}{RESET}")
}

pub fn green(text: &str) -> String {
    format!("{GREEN}{text}{RESET}")
}
