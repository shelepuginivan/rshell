use std::error::Error;
use std::env::var;
use std::fs::File;
use std::process::Stdio;
use std::str::SplitWhitespace;
use crate::ExecutionResult;
use crate::colors::*;
use crate::execute::execute_code;

pub fn parse_single_argument(argument: &str) -> Option<String> {
    return match argument.chars().next().unwrap() {
        '$' => {
            if argument.len() > 1 {
                let variable = argument.to_string().replace('$', "");

                let value = match var(&variable) {
                    Ok(value) => value,
                    Err(_) => variable
                };

                Some(value)
            } else {
                Some(String::from(argument))
            }
        },
        '\'' => Some(argument.replacen('\'', "", 1)),
        '#' => None,
        _ => Some(String::from(argument))
    }
}

pub fn parse_args(args: SplitWhitespace) -> Vec<String>
{
    let mut parsed_args: Vec<String> = Vec::new();

    for arg in args {
        match parse_single_argument(arg) {
            Some(parsed_value) => parsed_args.push(parsed_value),
            None => break
        }
    }

    parsed_args
}

pub fn generate_stdout(command: Option<&&str>) -> Result<(Stdio, bool), ExecutionResult> {
    let mut write_to_file = false;

    let stdio = if command.is_some() {
        let append = command.unwrap().starts_with("&a");
            let write = command.unwrap().starts_with("&w");

            let stdio = if append || write {
                let filename_untrimmed = command.unwrap().replace("&a ", "").replace("&w ", "");
                let filename = filename_untrimmed.trim();

                let file = match File::options()
                    .create(!append)
                    .append(append)
                    .write(write)
                    .truncate(write)
                    .open(filename) {
                        Ok(file) => file,
                        Err(err) => return Err(ExecutionResult::Error(Box::new(err)))
                    };

                write_to_file = true;
                Stdio::from(file)
            } else {
                Stdio::piped()
            };

            stdio
    } else {
        Stdio::inherit()
    };

    return Ok((stdio, write_to_file));
}

pub fn error_log(error: Box<dyn Error>) {
    eprintln!("{}: {error}", red("rsh"));
}

pub fn get_alias(alias: &str) -> String {
    println!("{alias}");

    if alias.starts_with('\'') {
        return alias.replacen('\'', "", 1);
    }

    match var("__ALIAS_".to_owned() + alias.trim()) {
        Ok(value) => value,
        Err(_) => alias.to_owned()
    }
}

pub fn is_function(name: &str) -> bool {
    var("__FN_".to_owned() + name).is_ok()
}

pub fn exec_function(function_name: &str, mut args: SplitWhitespace) -> ExecutionResult {
    let mut function_body = match var("__FN_".to_owned() + function_name) {
        Ok(code) => code,
        Err(err) => return ExecutionResult::Error(Box::new(err))
    };

    for token in function_body.clone().split_whitespace() {
        if token.starts_with('&') && !token.starts_with("&&") {
            match args.next() {
                Some(value) => function_body = function_body.replace(token.trim(), value),
                None => break
            };
        }
    };

    execute_code(&function_body)
}
