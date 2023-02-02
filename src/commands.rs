use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub output: String,
    pub status: i32,
}

fn output_as_string(output: Vec<u8>) -> Option<String> {
    match String::from_utf8(output) {
        Ok(s) => Some(s),
        _ => None,
    }
}

#[cfg(target_family = "windows")]
pub fn run_command(command: &str) -> CommandOutput {
    let command = Command::new("pwsh")
        .args(["-NoLogo", "-Command", command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Couldn't Run Command");

    let output: String = if command.status.success() {
        output_as_string(command.stdout.to_vec()).unwrap()
    } else {
        output_as_string(command.stderr.to_vec()).unwrap()
    };

    let status: i32 = command.status.code().unwrap_or(1);

    CommandOutput { output, status }
}

#[cfg(target_family = "unix")]
pub fn run_command(command: &str) -> CommandOutput {
    let command = Command::new("dash")
        .args(["-c", command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Couldn't Run Command");

    let output: String = if command.status.success() {
        output_as_string(command.stdout.to_vec()).unwrap()
    } else {
        output_as_string(command.stderr.to_vec()).unwrap()
    };

    let status: i32 = command.status.code().unwrap_or(1);

    CommandOutput { output, status }
}

#[cfg(test)]
mod tests {
    use crate::commands::{output_as_string, run_command, CommandOutput};

    #[test]
    fn test_output_as_string() {
        let hello: Vec<u8> = vec![104, 101, 108, 108, 111];
        assert!(output_as_string(hello).unwrap() == "hello");

        let world: Vec<u8> = vec![119, 111, 114, 108, 100];
        assert_eq!(output_as_string(world).unwrap(), "world");

        let hello_world: Vec<u8> = vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
        assert_eq!(output_as_string(hello_world).unwrap(), "hello world!");
    }

    #[test]
    fn test_output_as_string_errors() {
        let faulty: Vec<u8> = vec![0, 159, 146, 150];
        assert!(output_as_string(faulty).is_none());
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_run_command_win() {
        let output: CommandOutput = run_command("Write-Host 'Hello World!'");
        assert_eq!(output.status, 0);
        assert_eq!(output.output, "Hello World!\r\n");
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_run_command_errors_win() {
        let output: CommandOutput = run_command("exit 1");
        assert_eq!(output.status, 1);
        assert_eq!(output.output, "");

        let output = run_command("no-command");
        assert_eq!(output.status, 1);
        assert!(output
            .output
            .contains("The term 'no-command' is not recognized as a name of a cmdlet"));
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_run_command_unix() {
        let output: CommandOutput = run_command("echo 'Hello World!'");
        assert_eq!(output.status, 0);
        assert_eq!(output.output, "Hello World!\n");
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_run_command_errors_unix() {
        let output: CommandOutput = run_command("exit 1");
        assert_eq!(output.status, 1);
        assert_eq!(output.output, "");

        let output = run_command("no-command");
        assert_eq!(output.status, 127);
        assert!(output.output.contains("no-command: not found"));
    }
}
