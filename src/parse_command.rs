/// Parses whole command to separate commands
/// 
/// Example:
/// 
/// ```rust
/// let input = "echo hello && echo world; echo a > text.txt";
/// let commands = parse_input();
/// 
/// assert_eq!(commands, vec![vec![vec!["echo", "hello"], vec!["echo", "world"]], vec![vec!["echo", "a", ">", "text.txt"]]])
/// ```
/// 
pub fn parse_command(command: &str) -> Vec<Vec<Vec<&str>>> {
    let mut separate_commands = Vec::new();

    for independent_command in command.split(";").collect::<Vec<&str>>() {
        let mut independent_commands: Vec<Vec<&str>> = Vec::new();

        for single_command in independent_command.split("&&") {
            let single_command_tokens = single_command.split_whitespace().collect();
            independent_commands.push(single_command_tokens);
        }

        separate_commands.push(independent_commands);
    }
    separate_commands
}
