/// ascii-escape colors

pub const RED: &str  = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";

pub const RESET: &str = "\x1b[0m";


pub fn as_error(message: &str) -> String {
    format!("{}{}{}", RED, message, RESET)
}

pub fn as_success(message: &str) -> String {
    format!("{}{}{}", GREEN, message, RESET)
}
