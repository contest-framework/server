//! runs commands in a subshell

use crate::{Result, UserError};
use std::process::Command;

pub enum Outcome {
  TestPass(),
  TestFail(),
  NotFound(String),
}

pub fn run(command: &str) -> Result<Outcome> {
  println!("executing: {command}");
  let words = shellwords::split(command).map_err(|err| UserError::CannotSplitShellString {
    source: command.to_owned(),
    err: err.to_string(),
  })?;
  let (cmd, args) = words.split_at(1);
  Ok(match Command::new(&cmd[0]).args(args).status() {
    Err(_) => Outcome::NotFound(command.to_owned()),
    Ok(exit_status) => {
      if exit_status.success() {
        Outcome::TestPass()
      } else {
        Outcome::TestFail()
      }
    }
  })
}
