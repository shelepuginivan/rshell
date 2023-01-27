use core::str;
use std::error::Error;
use std::fs::File;
use std::process::{Child, Command, Stdio};
use crate::builtins;
use crate::utils;

pub enum ExecutionResult {
    Success,
    Error(Box<dyn Error>),
    Exit
}

pub fn execute(command_with_pipes: &str) -> ExecutionResult {
    if command_with_pipes.len() == 0 || command_with_pipes.starts_with("#") {
        return ExecutionResult::Success;
    }

    let binding = command_with_pipes
        .replace(">>", "| &a")
        .replace(">", "| &w");
        
    
    let mut commands = binding.split(" | ").peekable();


    let mut previous_command = None;

    while let Some(cmd) = commands.next() {
        let mut tokens = cmd.split_whitespace();

        let command = tokens.next().unwrap();
        let mut args = tokens;

        match command {
            "cd" => return builtins::change_directory(args.next()),

            "exit" => return ExecutionResult::Exit,

            "set" => return builtins::set_variable(args.next()),

            _ => {
                let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                let (stdout, write_to_file) = match utils::generate_stdout(commands.peek()) {
                    Ok((stdio, write)) => (stdio, write),
                    Err(execution_error) => return execution_error
                };

                let output = if write_to_file {
                    // write to the file for the first time
                    match Command::new(command)
                        .args(utils::parse_args(args))
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn() {
                            Ok(_) => {},
                            Err(err) => return ExecutionResult::Error(Box::new(err))
                        }

                    let mut previous_filename = commands.next().unwrap().replace("&a ", "").replace("&w ", "");
                    
                    while commands.peek().is_some() && commands.peek().unwrap().starts_with("&") {
                        let append = commands.peek().unwrap().starts_with("&a");
                        
                        let filename = commands.peek().unwrap().replace("&a ", "").replace("&w ", "");

                        let file = match File::options()
                            .create(!append)
                            .append(append)
                            .write(!append)
                            .truncate(!append)
                            .open(&filename) {
                                Ok(file) => file,
                                Err(err) => return ExecutionResult::Error(Box::new(err))
                            };

                        let file_stream = Stdio::from(file);
                        
                        // wait until data is written to the file (from previous file)
                        // for the first iteration it will be the file we wrote in
                        // before the cycle
                        match Command::new("cat")
                            .arg(previous_filename)
                            .stdout(file_stream)
                            .spawn()
                            .unwrap()
                            .wait() {
                                Ok(_) => {},
                                Err(err) => return ExecutionResult::Error(Box::new(err))
                            }

                        previous_filename = filename;
                        commands.next();
                    }

                    // after the cycle is finished, we have to output
                    // file to pipe if there are some other pipes
                    // and we have to output it to /dev/null
                    // if there are not
                    let stdout = if commands.peek().is_some() {Stdio::piped()} else {Stdio::null()};
                    
                    Command::new("cat")
                            .arg(previous_filename)
                            .stdout(stdout)
                            .spawn()
                } else {
                    Command::new(command)
                        .args(utils::parse_args(args))
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn()
                };

                match output {
                    Ok(output) => {
                        previous_command = Some(output);
                    },
                    Err(err) => return ExecutionResult::Error(Box::new(err))
                }
            }
        }
    }

    if previous_command.is_none() {
        return ExecutionResult::Success;
    }

    match previous_command.unwrap().wait() {
        Ok(_) => ExecutionResult::Success,
        Err(err) => ExecutionResult::Error(Box::new(err))
    }
}
