use std::env::{set_current_dir, set_var, var};
use std::error::Error;
use std::path::Path;
use crate::execute::ExecutionResult;
use crate::utils::parse_single_argument;

/// Implementation of shell built-in `cd` function
/// 
/// Changes current process directory
/// 
/// If no argument provided, changes directory to $HOME.
/// This also happens if argument is '~'
pub fn change_directory(directory: Option<&str>) -> ExecutionResult {
    let path = match directory {
        None => var("HOME").expect("rsh: unexpected internal error"),
        Some("~") => var("HOME").expect("rsh: unexpected internal error"),
        Some(path) => String::from(path)
    };

    return match set_current_dir(Path::new(&path)) {
        Ok(_) => ExecutionResult::Success,
        Err(err) => ExecutionResult::Error(Box::new(err))
    }
}

/// Implementation of shell built-in `set` function
/// 
/// Sets a variable
/// 
/// ```rsh
/// set key=value
/// ```
/// 
/// Created variable can be accessed with `$` symbol
/// 
/// In the above example:
/// ```rsh
///echo $key 
/// ```
/// 
/// will print `value` to stdout
pub fn set_variable(expression: Option<&str>) -> ExecutionResult {
    let mut args = match expression {
        Some(args) => args.split('='),
        None => return ExecutionResult::Error(Box::<dyn Error>::from("expression required"))
    };

    let key = parse_single_argument(args.next().unwrap().trim());

    let value = match args.next() {
        Some(value) => parse_single_argument(value.trim()),
        None => return ExecutionResult::Error(Box::<dyn Error>::from("expression required"))
    };

    let (parsed_key, parsed_value) = match (key, value) {
        (Some(key), Some(value)) => (key, value),
        (None, _) | (_, None) => return ExecutionResult::Error(Box::<dyn Error>::from("expression required"))
    };

    set_var(parsed_key, parsed_value);

    ExecutionResult::Success
}
