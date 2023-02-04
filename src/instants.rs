/// instant commands that support pipes, but
/// instantly return ExecutionResult.
/// They print result of the action directly
/// to the console (stdout).
/// 
/// In shell, this functions starts with `@` char.
/// For example, function `instant_exec` will be
/// `@exec`

use std::{process::Child, io::Read};
use crate::{execute::{ExecutionResult, execute_file}, colors::*};

pub fn instant_exec(stdin: Option<Child>, args: Vec<String>) -> ExecutionResult {
    let mut first_arg = String::new();

    match stdin {
        Some(process) => {
            match process.stdout.unwrap().read_to_string(&mut first_arg) {
                Ok(_) => {},
                Err(err) => return ExecutionResult::Error(Box::new(err))
            };
        },
        None => first_arg = args.to_owned().into_iter().next().unwrap_or_default()
    };

    let no_exit = args.contains(&"--noexit".to_string()) || args.contains(&"-n".to_string());

    match execute_file(first_arg.trim()) {
        ExecutionResult::Success => ExecutionResult::Success,
        ExecutionResult::Error(err) => ExecutionResult::Error(err),
        ExecutionResult::Exit => if no_exit {ExecutionResult::Success} else {ExecutionResult::Exit}
    }
}

pub fn instant_format(stdin: Option<Child>, args: Vec<String>) -> ExecutionResult {
    let mut first_arg = String::new();

    match stdin {
        Some(process) => {
            match process.stdout.unwrap().read_to_string(&mut first_arg) {
                Ok(_) => {},
                Err(err) => return ExecutionResult::Error(Box::new(err))
            };
        },
        None => {
            let mut string_to_format = String::new();

            for arg in args.to_owned().into_iter().peekable() {
                if arg.starts_with('-') {
                    break;
                }

                string_to_format = string_to_format + &(arg + " ");
            }

            first_arg = string_to_format
        }
    };

    first_arg = first_arg.trim_end().to_string();

    for arg in args.iter() {
        let format_modifier = match arg.as_str() {
            "--red" => RED,
            "--green" => GREEN,
            "--yellow" => YELLOW,
            "--blue" => BLUE,
            "--magenta" => MAGENTA,
            "--cyan" => CYAN,
            "--white" => WHITE,
            "--bold" => BOLD,
            "--dimmed" => DIMMED,
            "--italic" => ITALIC,
            "--underline" => UNDERLINE,
            _ => ""
        };

        first_arg = String::from(format_modifier) + &first_arg;
    }

    println!("{first_arg}");

    ExecutionResult::Success
}
