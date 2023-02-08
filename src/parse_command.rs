/// Parses whole command to separate commands
/// 
/// Example:
/// 
/// ```rust
/// let input = "echo hello && echo world; echo a > text.txt";
/// let commands = parse_commands(input);
/// 
/// assert_eq!(commands, vec![vec!["echo hello", "echo world"], vec![" echo a > text.txt"]])
/// ```
/// 
pub fn parse_command(command: &str) -> Vec<Vec<&str>> {
    let mut separate_commands: Vec<Vec<&str>> = Vec::new();

    for independent_command in command.split("; ").collect::<Vec<&str>>() {
        separate_commands.push(independent_command.split(" && ").collect());
    }
    separate_commands
}
