use crate::execute::*;

pub fn execute_code(code: &str) -> ExecutionResult {
    for line in code.split("\n") {
        match execute(line) {
            ExecutionResult::Success => continue,
            ExecutionResult::Error(err) => return ExecutionResult::Error(err),
            ExecutionResult::Exit => return ExecutionResult::Exit
        };
    }

    ExecutionResult::Success
}
