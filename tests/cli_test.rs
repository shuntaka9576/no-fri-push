use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_with_default_args() {
    let mut cmd = Command::cargo_bin("no-fri-push").unwrap();

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Don't push on Friday!"));
}

#[test]
fn test_with_custom_message() {
    let mut cmd = Command::cargo_bin("no-fri-push").unwrap();

    cmd.args(["--message", "Hello, testers!"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, testers!"));
}

#[test]
fn test_ascii_art_alignment() {
    let mut cmd = Command::cargo_bin("no-fri-push").unwrap();
    cmd.args(["--message", "TestAlign"]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8_lossy(&output);

    let lines_to_check = [r"\ (•◡•) /", r" \     /", "  --", "  |  |", "   |_ |_"];

    for line in &lines_to_check {
        assert!(
            output_str.contains(line),
            "Output did not contain the expected ASCII art line: {}",
            line
        );
    }
}
