use std::{
    io::Write,
    process::{
        Command,
        Stdio,
    },
};

use crate::{
    increment,
    sort,
    HistoryResults,
};

pub fn run(
    history: HistoryResults,
    input_lines: Vec<String>,
    cmd: Vec<&str>,
) -> HistoryResults {
    // let history = ;

    let mut arguments = cmd.into_iter().map(|f| f.to_string());
    let command = arguments.next().unwrap();

    let command_input = sort::run(history.clone(), input_lines)
        .0
        .into_iter()
        .map(|f| f.value)
        .collect();

    let child_output =
        shell_command(command, arguments.collect(), command_input);

    increment::run(history, child_output)
}

fn shell_command(
    command: String,
    args: Vec<String>,
    stdin: Vec<String>,
) -> Vec<String> {
    let mut child = Command::new(&command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            panic!("failed to spawn program {}: {}", command, e)
        });
    {
        let mut child_stdin = child
            .stdin
            .take()
            .unwrap_or_else(|| panic!("failed to get stdin of {}", command));
        for line in &stdin {
            let text = format!("{}\n", line);
            child_stdin.write_all(text.as_bytes()).ok();
        }
    }
    let child_output_bytes = child
        .wait_with_output()
        .unwrap_or_else(|e| {
            panic!("failed to get stdout of {}: {}", command, e)
        })
        .stdout;
    let child_output = std::str::from_utf8(&child_output_bytes)
        .unwrap_or_else(|e| {
            panic!("failed to read output of {}: {}", command, e)
        })
        .lines()
        .map(|i| i.to_string())
        .collect();
    child_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_shell() {
        let command = "cat".to_string();
        let input = "hello\nworld!".lines().map(|x| x.to_string()).collect();
        let expected_output: Vec<String> =
            "hello\nworld!".lines().map(|x| x.to_string()).collect();

        assert_eq!(shell_command(command, Vec::new(), input), expected_output)
    }
}
