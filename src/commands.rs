use std::process::{Command, Stdio};

// Run the commands
fn output_as_string(output: Vec<u8>) -> Option<String> {
    match String::from_utf8(output) {
        Ok(s) => Some(s),
        _ => None,
    }
}

pub fn run_command(command: &str) -> (String, i32) {
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

    (output, status)
}
