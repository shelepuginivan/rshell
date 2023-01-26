mod colors;
mod parse_command;
mod execute;
mod execute_code;

use std::env;
use std::io::Read;
use std::process::exit;
use std::error::Error;
use std::fs::File;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use execute::*;
use colors::*;
use execute_code::execute_code;
use parse_command::parse_command;

fn main() {
    let home: String = env::var("HOME").unwrap();
    let history_path: &str = &format!("{home}/.rsh_history");
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

    if env::args().len() > 1 {
        let filename = match env::args().nth(1) {
            Some(filename) => filename,
            None => exit(22)
        };

        let mut code = String::new();
        let mut file = match File::options()
            .read(true)
            .open(std::path::Path::new(&filename)) {
            Ok(file) => file,
            Err(err) => {
                error_log(Box::new(err));
                exit(2)
            }
        };

        match file.read_to_string(&mut code) {
            Ok(_) => {},
            Err(err) => {
                error_log(Box::new(err));
                exit(5)
            }
        }
        
        match execute_code(&code) {
            ExecutionResult::Error(err) => {
                error_log(err);
                exit(1)
            },
            _ => exit(0)
        }
    }

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
                println!("{rsh_internal_err}");
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

    format!("{exit_status_indicator} {working_directory} $ ")
}

fn error_log(error: Box<dyn Error>) {
    eprintln!("{}: {error}", red("rsh"));
}
