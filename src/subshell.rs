//! runs commands in a subshell

use crate::{Result, UserError};
use std::process::Command;

pub enum Outcome {
  TestPass,
  TestFail,
}

pub fn run(command: &str) -> Result<Outcome> {
  println!("executing: {command}");
  let words = shellwords::split(command).map_err(|err| UserError::CannotSplitShellString {
    source: command.to_owned(),
    err: err.to_string(),
  })?;
  let ([cmd, ..], args) = words.split_at(1) else {
    return Err(UserError::RunCommandIsEmpty);
  };
  match Command::new(cmd).args(args).status() {
    Err(_) => Err(UserError::RunCommandNotFound { command: cmd.clone() }),
    Ok(exit_status) => {
      if exit_status.success() {
        Ok(Outcome::TestPass)
      } else {
        Ok(Outcome::TestFail)
      }
    }
  }
}
