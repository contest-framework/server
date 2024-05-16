//! commands received from the client (through the FIFO)

use super::fifo_data::FifoTrigger;
use crate::Result;
use std::fmt::Display;

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
      Trigger::TestAll => f.write_str("testAll"),
      Trigger::TestFile { file } => write!(f, "testFile {file}"),
      Trigger::TestFileLine { file, line } => write!(f, "testFunction {file}:{line}"),
      Trigger::RepeatLastTest => f.write_str("repeatTest"),
    }
  }
}

impl Trigger {
  pub fn parse(line: &str) -> Result<Self> {
    FifoTrigger::parse(line)?.into_trigger()
  }
}
