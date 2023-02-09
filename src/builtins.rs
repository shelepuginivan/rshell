use std::env::{set_current_dir, set_var, var, current_dir};
use std::error::Error;
use std::io::{Write, stdout};
use std::path::Path;
use std::str::SplitWhitespace;
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

/// Alias implementation
/// 
/// Creates an alias
/// ```rsh
/// alias cat=bat
/// ```
/// 
/// Now bat will be executed instead of cat
/// 
/// To execute the exact command, use literal strings (e.g. `'cat`)
pub fn set_alias(expression: Option<&str>) -> ExecutionResult {
    let mut args = match expression {
        Some(args) => args.split('='),
        None => return ExecutionResult::Error(Box::<dyn Error>::from("expression required"))
    };

    let name = match args.next() {
        Some(name) => "__ALIAS_".to_owned() + name.trim(),
        None => return ExecutionResult::Error(Box::<dyn Error>::from("expression required"))
    };

    let value = match args.next() {
        Some(value) =>value.trim(),
        None => return ExecutionResult::Error(Box::<dyn Error>::from("expression required"))
    };

    set_var(name, value);

    ExecutionResult::Success
}

pub fn function_declaration(args: SplitWhitespace) -> ExecutionResult {
    let mut tokens = args;

    let function_name = match tokens.next() {
        Some(name) => name,
        None => return ExecutionResult::Error(Box::<dyn Error>::from("function name required"))
    };

    let mut function_body = String::new();

    if tokens.clone().peekable().peek().unwrap() == &"{" {
        tokens.next();
        
        let cwd_path = current_dir().unwrap_or_default();
        let cwd = cwd_path.to_str().unwrap_or_default();

        let prompt = " ".repeat(cwd.len() + 3) + "> ";

        print!("{prompt}");

        loop {
            match stdout().flush() {
                Ok(_) => {},
                Err(err) => return ExecutionResult::Error(Box::from(err))
            };

            let mut line = String::new();

            print!("{prompt}");

            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
                    if line.trim() == "}" {
                        match stdout().flush() {
                            Ok(_) => {},
                            Err(err) => return ExecutionResult::Error(Box::from(err))
                        };
                        
                        break;
                    };
        
                    function_body += &line;
                },
                Err(err) => return ExecutionResult::Error(Box::from(err))
            };
        }
    } else {
        function_body = tokens.collect::<Vec<&str>>().join(" ");
    };

    set_var("__FN_".to_owned() + function_name, function_body);

    return ExecutionResult::Success;
}
