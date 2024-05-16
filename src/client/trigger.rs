//! commands received from the client (through the FIFO)

use crate::Result;
use std::fmt::Display;

use super::fifo_data::FifoTrigger;

#[derive(Debug, Eq, PartialEq)]
pub enum Trigger {
  TestAll,
  TestFile { file: String },
  TestFileLine { file: String, line: String },
  RepeatLastTest,
}

impl Display for Trigger {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Trigger::TestAll => f.write_str("run all tests"),
      Trigger::TestFile { file } => write!(f, "test {file}"),
      Trigger::TestFileLine { file, line } => write!(f, "test {file}:{line}"),
      Trigger::RepeatLastTest => f.write_str("repeat the last test"),
    }
  }
}

impl Trigger {
  pub fn parse(line: &str) -> Result<Self> {
    FifoTrigger::parse(line)?.into_trigger()
  }
}
