use std::process::{Command, Output};

fn newpass(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_newpass"))
        .args(args)
        .output()
        .expect("failed to run newpass")
}

fn stdout_lines(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

#[test]
fn no_arguments_prints_default_password() {
    let output = newpass(&[]);
    assert!(output.status.success());
    let lines = stdout_lines(&output);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].len(), 16);
}

#[test]
fn one_password_per_length() {
    let output = newpass(&["8", "24"]);
    assert!(output.status.success());
    let lengths: Vec<usize> = stdout_lines(&output).iter().map(String::len).collect();
    assert_eq!(lengths, vec![8, 24]);
}

#[test]
fn invalid_argument_fails_without_partial_output() {
    let output = newpass(&["16", "abc"]);
    assert!(!output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert!(String::from_utf8_lossy(&output.stderr).contains("'abc'"));
}

#[test]
fn oversized_length_is_rejected() {
    let output = newpass(&["18446744073709551615"]);
    assert!(!output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn help_succeeds() {
    let output = newpass(&["--help"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Usage"));
}
