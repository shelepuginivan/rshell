mod colors;
mod parse_command;
mod execute;

use std::env;
use std::process::exit;
use std::error::Error;
use std::fs::File;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use execute::*;
use colors::*;
use parse_command::parse_command;

fn main() {
    let home: String = env::var("HOME").unwrap();
    let history_path: &str = &format!("{}/.rsh_history", home);
    // TODO: implement config parser
    // let config_path: String = format!("{}/.rshrc", home);
    let rsh_internal_err = format!("{}: unexpected internal error", red("rsh"));

    let mut rl = Editor::<()>::new()
        .expect(&rsh_internal_err);

    // load history and if it doesn't exist, creates new history file
    if rl.load_history(&history_path).is_err() {
        File::create(history_path).expect(&format!("{}: failed to create history file", red("rsh")));
    }

    let mut previous_command_succeed = true; 

    loop
    {
        let working_directory = env::current_dir()
            .expect(&rsh_internal_err);

        let prompt = generate_prompt(previous_command_succeed, working_directory.display());

        previous_command_succeed = true;

        let input = match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            },
            Err(ReadlineError::Interrupted) => {
                previous_command_succeed = false;
                continue;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(_) => {
                println!("{}", rsh_internal_err);
                break;
            }
        };

        let separate_commands = parse_command(&input);

        for separate_command in separate_commands {
            for command_with_pipes in separate_command {
                previous_command_succeed = match execute(command_with_pipes) {
                    ExecutionResult::Success => true,
                    ExecutionResult::Error(err) => {
                        error_log(err);
                        false
                    },
                    ExecutionResult::Exit => {
                        rl.save_history(&history_path).unwrap();
                        exit(0)
                    }
                };
                if !previous_command_succeed {
                    break
                }
            }
        }
    }

    rl.save_history(&history_path)
        .expect(&format!("{}: failed to save history", red("rsh")));
}

fn generate_prompt(previous_command_succeed: bool, working_directory: std::path::Display) -> String {
    // TODO: generate prompt from config parameters
    // e.g. there is parameter 'indecator_symbol' in config
    // and prompt is generated with this parameter value

    let exit_status_indicator = if previous_command_succeed {
        green("*")
    } else {
        red("*")
    };

    format!("{} {} $ ", exit_status_indicator, working_directory)
}

fn error_log(error: Box<dyn Error>) {
    eprintln!("{}: {error}", red("rsh"));
}
