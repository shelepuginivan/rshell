use core::str;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio, exit};
use crate::builtins;
use crate::instants::*;
use crate::parse_command::parse_command;
use crate::utils::*;

pub enum ExecutionResult {
    Success,
    Error(Box<dyn Error>),
    Exit
}

pub fn execute(command_with_pipes: &str) -> ExecutionResult {
    if command_with_pipes.len() == 0 || command_with_pipes.starts_with("#") {
        return ExecutionResult::Success;
    }

    let mut binding = command_with_pipes
        .replace(">>", "| &a")
        .replace(">", "| &w");
    
    for token in binding.clone().replace('=', " ").split_whitespace() {
        if token.starts_with('\'') {
            continue
        }

        binding = binding.replace(token, &get_alias(token));
    }
    
    let mut commands = binding.split(" | ").peekable();


    let mut previous_command = None;

    while let Some(cmd) = commands.next() {
        let mut tokens = cmd.split_whitespace();

        let temp_command = get_alias(tokens.next().unwrap());
        let command = temp_command.as_str();
        
        let mut args = tokens;

        match command {
            "cd" => return builtins::change_directory(args.next()),

            "exit" => return ExecutionResult::Exit,

            "set" => return builtins::set_variable(args.next()),

            "alias" => return builtins::set_alias(Some(&args.collect::<Vec<&str>>().join(" "))),

            "fn" => return builtins::function_declaration(args),

            _ => {
                if is_function(command) {
                    return exec_function(command, args);
                }

                if command.starts_with('@') {
                    return match command {
                        "@exec" => instant_exec(previous_command, parse_args(args)),
                        "@format" => instant_format(previous_command, parse_args(args)),
                        _ => return ExecutionResult::Success
                    }
                }


                let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                let (stdout, write_to_file) = match generate_stdout(commands.peek()) {
                    Ok((stdio, write)) => (stdio, write),
                    Err(execution_error) => return execution_error
                };

                let output = if write_to_file {
                    // write to the file for the first time
                    match Command::new(command)
                        .args(parse_args(args))
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
                    unsafe {
                        Command::new(command)
                            .pre_exec(|| {
                                libc::signal(libc::SIGINT, libc::SIG_DFL);
                                libc::signal(libc::SIGQUIT, libc::SIG_ERR);
                                Ok(())
                            })
                            .args(parse_args(args))
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn()
                    }
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

pub fn execute_code(code: &str) -> ExecutionResult {
    for line in code.split("\n") {
        let separate_commands = parse_command(line);
        for separate_command in separate_commands {
            for command_with_pipes in separate_command{
                match execute(command_with_pipes) {
                    ExecutionResult::Success => continue,
                    ExecutionResult::Error(err) => {
                        error_log(err);
                        break;
                    },
                    ExecutionResult::Exit => return ExecutionResult::Exit
                };
            }
            
        }
    }

    ExecutionResult::Success
}

pub fn execute_file<P>(path: P) -> ExecutionResult
where P: AsRef<Path> {
    let mut code = String::new();
    let mut file = match File::options()
            .read(true)
            .open(path) {
            Ok(file) => file,
            Err(err) => {
                error_log(Box::new(err));
                return ExecutionResult::Exit;
            }
        };

    match file.read_to_string(&mut code) {
        Ok(_) => {},
        Err(err) => {
            error_log(Box::new(err));
            exit(5)
        }
    }

    execute_code(&code)
}
