use std::fs::File;
use std::process::{Child, Command, Stdio, exit};
use std::env::{set_current_dir, var};
use std::path::Path;


pub fn execute(raw_command: &str) -> bool {
    if raw_command.len() == 0 {
        return true;
    }

    let binding = raw_command
        .replace(">>", "| &a")
        .replace(">", "| &w");
        

    let mut write_to_file;
    
    let mut commands = binding.split(" | ").peekable();


    let mut previous_command = None;

    while let Some(cmd) = commands.next() {
        write_to_file = false;

        let mut tokens = cmd.split_whitespace();

        let command = tokens.next().unwrap();
        let mut args = tokens;

        match command {
            "cd" => {
                let path = match args.next() {
                    Some("~") => var("HOME").expect("rsh: unexpected internal error"),
                    Some(dir) => String::from(dir),
                    None => var("HOME").expect("rsh: unexpected internal error")
                };

                match set_current_dir(Path::new(&path)) {
                    Ok(_) => return true,
                    Err(err) => {
                        eprintln!("rsh: {}", err);
                        return false;
                    }
                };
            },

            "exit" => exit(0),

            _ => {
                let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                let stdout = if commands.peek().is_some() {
                    let append = commands.peek().unwrap().starts_with("&a");
                    let write = commands.peek().unwrap().starts_with("&w");
                    let stdio = if append || write {
                        let filename_raw = commands.peek().unwrap().replace("&a ", "").replace("&w ", "");
                        let filename = filename_raw.trim();
                        let file = match File::options()
                            .create(!append)
                            .append(append)
                            .write(write)
                            .truncate(write)
                            .open(filename) {
                                Ok(file) => file,
                                Err(err) => {
                                    eprintln!("rsh: {err}");
                                    return false;
                                }
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

                let output = if write_to_file {
                    // write to the file for the first time
                    match Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn() {
                            Ok(_) => {},
                            Err(err) => {
                                eprintln!("rsh: {err}");
                                return false;
                            }
                        }

                    let previous_filename_raw = commands.next().unwrap().to_owned();
                    let mut previous_filename = previous_filename_raw.replace("&a ", "").replace("&w ", "");
                    
                    while commands.peek().is_some() && commands.peek().unwrap().starts_with("&") {
                        let append = commands.peek().unwrap().starts_with("&a");
                        let write = commands.peek().unwrap().starts_with("&w");
                        
                        let filename_raw = commands.peek().unwrap();
                        let filename = filename_raw.replace("&a ", "").replace("&w ", "");

                        let file = match File::options()
                            .create(!append)
                            .append(append)
                            .write(write)
                            .truncate(write)
                            .open(&filename) {
                                Ok(file) => file,
                                Err(err) => {
                                    eprintln!("rsh: {err}");
                                    return false;
                                }
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
                                Err(err) => {
                                    eprintln!("{err}");
                                    return false;
                                }
                            }

                        previous_filename = filename;
                        commands.next();
                    }

                    // after the cycle is finished, we have to output
                    // file to pipe if there are some other pipes
                    // and we have to output it to /dev/null
                    // if there are not
                    let has_pipes_after = commands.peek().is_some();
                    let stdout = if has_pipes_after {Stdio::piped()} else {Stdio::null()};

                    Command::new("cat")
                            .arg(previous_filename)
                            .stdout(stdout)
                            .spawn()
                } else {
                    Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn()
                };
                match output {
                    Ok(output) => {
                        previous_command = Some(output);
                    },
                    Err(e) => {
                        previous_command = None;
                        eprintln!("rsh: {}", e);
                    },
                };
            }
        }
    }

    if let Some(mut final_command) = previous_command {
        final_command.wait().is_ok()
    } else {
        false
    }
}
