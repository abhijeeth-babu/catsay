use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary doesn't exist")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
}

#[test]
fn no_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary doesn't exist")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn cat_is_dead() {
    Command::cargo_bin("catsay")
        .expect("binary doesnt exist")
        .args(&["-d"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{02e3}"));
}

#[test]
fn cat_can_read() {
    let mut child = Command::cargo_bin("catsay")
        .expect("binary doesn't exist")
        .args(&["-i"])
        .stdin(Stdio::piped()) // Set stdin to use a pipe
        .stdout(Stdio::piped()) // Set stdout to use a pipe
        .spawn()
        .expect("Unable to create child process");

    // Write "hello world" to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(b"hello world\n")
            .expect("failed to write to stdin");
    } else {
        panic!("failed to open stdin");
    }

    // Assert that the output contains "hello world"
    let output = child.wait_with_output().expect("failed to wait on child");

    let stdout_str =
        std::str::from_utf8(&output.stdout).expect("failed to convert stdout to string");

    assert!(stdout_str.contains("hello world"));
}
