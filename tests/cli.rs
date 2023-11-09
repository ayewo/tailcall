use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use rexpect::spawn;
use rexpect::error::*;

#[cfg(test)]
mod usage {
  use super::*;
  #[test]
  fn empty_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tailcall")?;

    cmd.arg("");
    cmd
      .assert()
      .failure()
      .stderr(predicate::str::contains("Usage: tailcall"));

    Ok(())
  }
}

// Check command tests
#[cfg(test)]
mod check {
  use super::*;
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
      .stderr(predicate::str::contains("Error: No such file or directory"));

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

    cmd
      .arg("check")
      .arg("examples/jsonplaceholder.graphql")
      .arg("--n-plus-one-queries")
      .arg("--schema");
    cmd
      .assert()
      .success()
      .stdout(predicate::str::contains("No errors found"));

    Ok(())
  }

  #[test]
  fn check_command_file_exists_and_valid2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tailcall")?;
    cmd
      .arg("check")
      .arg("examples/jsonplaceholder.graphql")
      .arg("-n")
      .arg("-s");
    cmd
      .assert()
      .success()
      .stdout(predicate::str::contains("No errors found"));

    Ok(())
  }

  #[test]
  fn check_command_file_exists_and_valid3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tailcall")?;
    cmd.arg("check").arg("examples/jsonplaceholder.graphql").arg("-s");
    cmd
      .assert()
      .success()
      .stdout(predicate::str::contains("No errors found"));

    Ok(())
  }
}

// Start command tests
#[cfg(test)]
mod start {
  use super::*;
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
}

// Init command tests
#[cfg(test)]
mod init {
use super::*;

  #[test]
  fn empty_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::Command::cargo_bin("tailcall")?;

    cmd.arg("init");
    cmd.assert().failure().stderr(predicates::prelude::predicate::str::contains(
      "error: the following required arguments were not provided:",
    ));
    cmd
      .assert()
      .failure()
      .stderr(predicates::prelude::predicate::str::contains("Usage: tailcall init <FILE_PATH>"));

    Ok(())
  }

  #[test]
//fn folder_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
  fn folder_nonexistent() -> Result<(), rexpect::error::Error> {
    let mut p = rexpect::spawn("cargo run -- init tmp0", Some(5000))?;
    let res = p.exp_regex(r#".*Do you want to add a file to the project\?.*"#)?;
    println!("Response: {:?}", res);

    let code = p.send("N\n")?; // you have to send the newline in test mode because it can't do the live readline checks that it can in a terminal
    println!("Response after sending 'N': {:?}", code);
    p.exp_regex(".*No such file or directory.*")?;
    println!("Response after res: {:?}", res);

    Ok(())
  }
  
  #[test]
  fn test_my_cmd() -> Result<(), rexpect::error::Error> {
    println!("Starting...");
    let mut p = rexpect::spawn("cargo run", Some(5000))?;
    let res = p.exp_regex(r#".*Are you sure\?.*"#)?;
    println!("Res after are you sure: {:?}", res);
    // you'll get this because it's doing the build and start:
    // ```
    // ("\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m   Compiling\u{1b}[0m test-rs-expect v0.1.0 (/Users/yaleman/Projects/test-rs-expect)\r\n\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m    Finished\u{1b}[0m dev [unoptimized + debuginfo] target(s) in 0.10s\r\n\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m     Running\u{1b}[0m `target/debug/test-rs-expect`\r\nHello, world!\r\n", "Are you sure? [y/n] \u{1b}[?25l")
    // ```
    let res = p.send("y\n")?; // you have to send the newline in test mode because it can't do the live readline checks that it can in a terminal
    println!("Res after sending y: {:?}", res);
    p.exp_regex(".*Result: Ok\\(true\\).*")?;
    println!("Res after res: {:?}", res);
    
    Ok(())
  }

  // #[test]
  // fn folder_nonexistent2() -> Result<(), Box<dyn std::error::Error>> {
  //   let mut p = rexpect::spawn("cargo run -- init tmp0", Some(5000))?;
  //   let mut res = p.exp_regex(r#".*Do you want to add a file to the project?.*"#)?;
  //   println!("Response: {:?}", res);

  //   let code = p.send("y\n")?; // you have to send the newline in test mode because it can't do the live readline checks that it can in a terminal
  //   println!("Response code after sending 'y': {:?}", code);
  //   res = p.exp_regex(".*Enter the file name:*")?;
  //   // code = p.send("n\n")?;
  //   println!("Response after last interaction: {:?}\n", res);
  //   res = p.exp_regex(".*Do you want to create the file .graphql?*")?;
  //   println!("Response after last interaction: {:?}\n", res);
  //   res = p.exp_regex(".*No such file or directory*")?;
  //   println!("Response after last interaction: {:?}\n", res);

  //   Ok(())
  // }

  // #[test]
  // fn folder_exists() -> Result<(), Box<dyn std::error::Error>> {
  //   let mut p = rexpect::spawn("cargo run -- init tmp", Some(5000))?;
  //   let mut res = p.exp_regex(r#".*Do you want to add a file to the project?.*"#)?;
  //   println!("Response: {:?}", res);

  //   let code = p.send("n\n")?; // you have to send the newline in test mode because it can't do the live readline checks that it can in a terminal
  //   println!("Response after sending 'N': {:?}", code);
  //   res = p.exp_regex(".*No such file or directory.*")?;
  //   println!("Response after res: {:?}", res);

  //   Ok(())
  // }
}
