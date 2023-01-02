use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn file_name_to_path(s: &str) -> String {
    return format!("tests/examples/error_checks/{}.ling", s);
}

#[test]
fn keyword_as_var() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("keyword_as_var"));
    cmd.assert().success().stdout(
        predicate::str::contains("keyword").and(predicate::str::contains("used as variable")),
    );

    Ok(())
}

#[test]
fn no_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("no_main"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("main procedure not found"));

    Ok(())
}

#[test]
fn multiple_mains() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("multiple_mains"));
    cmd.assert().success().stdout(predicate::str::contains(
        "multiple procedures with name \"main\"",
    ));

    Ok(())
}

#[test]
fn missing_semicolon() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("missing_semicolon"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected").and(predicate::str::contains(";")));

    Ok(())
}

#[test]
fn invalid_escape_sequence() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("invalid_escape_sequence"));
    cmd.assert().success().stdout(
        predicate::str::contains("invalid escape sequence").and(predicate::str::contains("\\f")),
    );

    Ok(())
}

#[test]
fn multiple_same_name_procs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linger")?;

    cmd.arg(file_name_to_path("multiple_same_name_procs"));
    cmd.assert().success().stdout(predicate::str::contains(
        "multiple procedures with name \"foo\"",
    ));

    Ok(())
}
