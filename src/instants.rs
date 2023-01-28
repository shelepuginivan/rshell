/// instant commands that support pipes, but
/// instantly return ExecutionResult.
/// They print result of the action directly
/// to the console (stdout).
/// 
/// In shell, this functions starts with `@` char.
/// For example, function `instant_exec` will be
/// `@exec`

use std::{process::Child, io::Read};
use crate::execute::{ExecutionResult, execute_file};

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
