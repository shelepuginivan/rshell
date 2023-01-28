use std::error::Error;
use std::env::var;
use std::fs::File;
use std::process::Stdio;
use std::str::SplitWhitespace;
use crate::ExecutionResult;
use crate::colors::*;

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
