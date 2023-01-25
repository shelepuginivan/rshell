use std::env;
use std::fs::File;
use rustyline::Editor;
use rustyline::error::ReadlineError;


mod colors;
mod parse_command;
mod execute;


fn main() {
    let home: String = env::var("HOME").unwrap();
    let history_path: &str = &format!("{}/.rsh_history", home);
    // TODO: implement config parser
    // let config_path: String = format!("{}/.rshrc", home);
    let rsh_err_log: String = colors::as_error("rsh");
    let rsh_internal_err = format!("{}: unexpected internal error", rsh_err_log);

    let mut rl = Editor::<()>::new()
        .expect(&rsh_internal_err);

    // load history and if it doesn't exist, creates new history file
    if rl.load_history(&history_path).is_err() {
        File::create(history_path).expect(&format!("{}: failed to create history file", rsh_err_log));
        println!("");
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

        let separate_commands = parse_command::parse_command(&input);

        for separate_command in separate_commands {
            for single_command_tokens in separate_command {
                previous_command_succeed = execute::execute(single_command_tokens);
                if !previous_command_succeed {
                    break
                }
            }
        }
    }

    rl.save_history(&history_path)
        .expect(&format!("{}: failed to save history", rsh_err_log));
}

fn generate_prompt(previous_command_succeed: bool, working_directory: std::path::Display) -> String {
    // TODO: generate prompt from config parameters
    // e.g. there is parameter 'indecator_symbol' in config
    // and prompt is generated with this parameter value

    let exit_status_indicator = if previous_command_succeed {
        colors::as_success("*")
    } else {
        colors::as_error("*")
    };

    format!("{} {} $ ", exit_status_indicator, working_directory)
}
