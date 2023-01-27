use std::env::var;
use std::str::SplitWhitespace;

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
