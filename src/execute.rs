use std::process::{Child, Command, Stdio, exit};
use std::env::{set_current_dir, var};
use std::path::Path;


pub fn execute(raw_command: &str) -> bool {
    if raw_command.len() == 0 {
        return true;
    }

    let mut commands = raw_command.split(" | ").peekable();
    let mut previous_command = None;

    while let Some(cmd) = commands.next() {
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
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

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