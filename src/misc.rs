use std::process::Command;

#[cfg(target_os = "linux")]
pub fn execute_command(command: &str) {
    let mut command = command.split_whitespace();
    let args = command.clone().skip(1);
    let command = command.next().unwrap();
    let output = Command::new(command).args(args).output().unwrap();
    if !output.status.success() {
        panic!(
            "Command {} failed with status code {}",
            command,
            output.status.code().unwrap()
        );
    }
}

#[cfg(not(target_os = "linux"))]
pub fn execute_command(command: &str) {
    println!("!!!DRY RUN: {}", command);
}
