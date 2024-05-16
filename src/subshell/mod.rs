//! runs commands in a subshell

use std::process::Command;

pub enum Outcome {
  TestPass(),
  TestFail(),
  NotFound(String),
}

pub fn run(command: &str) -> Outcome {
  println!("executing: {command}");
  let argv = shellwords::split(command).unwrap();
  let (cmd, args) = argv.split_at(1);
  match Command::new(&cmd[0]).args(args).status() {
    Err(_) => Outcome::NotFound(command.to_owned()),
    Ok(exit_status) => {
      if exit_status.success() {
        Outcome::TestPass()
      } else {
        Outcome::TestFail()
      }
    }
  }
}
