use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

fn run(args: &[&str], expected: String) -> TestResult {
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "Hello there".to_string())
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "Hello there".to_string())
}

#[test]
fn hello1_no_new_line() -> TestResult {
    run(&["Hello there", "-n"], "Hello there".to_string())
}

#[test]
fn hello2_no_new_line() -> TestResult {
    run(&["Hello", "there", "-n"], "Hello there".to_string())
}
