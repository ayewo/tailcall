use std::process::Command;

use assert_cmd::prelude::*;
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
fn check_command_file_exists_but_is_invalid() -> Result<(), Box<dyn std::error::Error>> {
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
