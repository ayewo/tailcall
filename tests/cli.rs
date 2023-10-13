use std::process::Command;

use assert_cmd::prelude::*;
use ntest::timeout;
use predicates::prelude::*;

#[test]
fn empty_command_args() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("");
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Usage: tailcall"));

  Ok(())
}

// Check command tests
#[test]
fn check_command_file_not_specified() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("check");
  cmd.assert().failure().stderr(predicate::str::contains(
    "error: the following required arguments were not provided",
  ));
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Usage: tailcall check <FILE_PATH>"));

  Ok(())
}

#[test]
fn check_command_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("check").arg("test.file.doesnt.exist.graphql");
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Failed to read file"));

  Ok(())
}

#[test]
fn check_command_file_exists_and_invalid() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd
    .arg("check")
    .arg("tests/graphql/errors/test-const-with-inline.graphql");
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error: Validation Error"));

  Ok(())
}

#[test]
fn check_command_file_exists_and_valid() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("check").arg("tests/graphql/valid/jsonplaceholder.graphql").arg("--n-plus-one-queries").arg("--schema");
  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("No errors found"));

  Ok(())
}

// Start command tests
#[test]
fn start_command_file_not_specified() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("start");
  cmd.assert().failure().stderr(predicate::str::contains(
    "error: the following required arguments were not provided",
  ));
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Usage: tailcall start <FILE_PATH>"));

  Ok(())
}

#[test]
fn start_command_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("start").arg("test.file.doesnt.exist.graphql");
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error: No such file or directory"));

  Ok(())
}

#[test]
fn start_command_log_level() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("start").arg("--log_level");
  cmd.assert().failure().stderr(predicate::str::contains(
    "error: unexpected argument '--log_level' found",
  ));
  cmd.assert().failure().stderr(predicate::str::contains(
    "Usage: tailcall start <FILE_PATH|--log-level <LOG_LEVEL>>",
  ));

  Ok(())
}

#[test]
fn start_command_log_level2() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("start").arg("--log-level");
  cmd.assert().failure().stderr(predicate::str::contains(
    "error: a value is required for '--log-level <LOG_LEVEL>' but none was supplied",
  ));
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("For more information, try '--help'"));

  Ok(())
}

#[test]
fn start_command_log_level3() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd.arg("start").arg("--log-level").arg("DEBUG");
  cmd.assert().failure().stderr(predicate::str::contains(
    "error: the following required arguments were not provided",
  ));
  cmd.assert().failure().stderr(predicate::str::contains(
    "Usage: tailcall start --log-level <LOG_LEVEL> <FILE_PATH>",
  ));

  Ok(())
}

#[test]
fn start_command_file_exists_and_invalid() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tailcall")?;

  cmd
    .arg("start")
    .arg("tests/graphql/errors/test-const-with-inline.graphql");
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error: Invalid Configuration"));

  Ok(())
}

#[test]
#[timeout(2000)]
// #[should_panic]
fn start_command_file_exists_and_valid() -> Result<(), String> {
  let mut cmd = match Command::cargo_bin("tailcall") {
    Ok(cmd) => cmd,
    Err(err) => return Err(format!("Failed to execute command: {}", err)),
  };

  let mut child = cmd
    .arg("start")
    .arg("tests/graphql/valid/jsonplaceholder.graphql")
    .spawn()
    .expect("Failed to spawn command");

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error: Server Failed"))
    .stderr(predicate::str::contains("The port is already in use"));

  child.kill().expect("Failed to kill command");

  Ok(())
}
