//! commands received from the client (through the FIFO)

use super::fifo_data::FifoTrigger;
use crate::UserError;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub enum Trigger {
  TestAll,
  TestFile { file: String },
  TestFileLine { file: String, line: usize },
  CustomCommand { run: String },
  RepeatLastTest,
}

impl Display for Trigger {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Trigger::TestAll => f.write_str("testAll"),
      Trigger::TestFile { file } => write!(f, "testFile {file}"),
      Trigger::TestFileLine { file, line } => write!(f, "testFileLine {file}:{line}"),
      Trigger::CustomCommand { run } => write!(f, "customCommand {run}"),
      Trigger::RepeatLastTest => f.write_str("repeatTest"),
    }
  }
}

impl TryFrom<FifoTrigger> for Trigger {
  type Error = UserError;

  fn try_from(fifo: FifoTrigger) -> std::result::Result<Self, Self::Error> {
    match fifo.command.to_ascii_lowercase().as_str() {
      "testall" => Ok(Trigger::TestAll),
      "repeattest" => Ok(Trigger::RepeatLastTest),
      "customcommand" => match fifo.run {
        Some(run) => Ok(Trigger::CustomCommand { run }),
        None => Err(UserError::MissingRunInTrigger),
      },
      "testfile" => match fifo.file {
        Some(file) => Ok(Trigger::TestFile { file }),
        None => Err(UserError::MissingFileInTrigger),
      },
      "testfileline" => match (fifo.file, fifo.line) {
        (Some(file), Some(line)) => Ok(Trigger::TestFileLine { file, line }),
        (None, Some(_)) => Err(UserError::MissingFileInTrigger),
        (Some(_), None) => Err(UserError::MissingLineInTrigger),
        (None, None) => Err(UserError::MissingFileAndLineInTrigger),
      },
      _ => Err(UserError::UnknownTrigger { source: fifo.command }),
    }
  }
}

impl TryFrom<&str> for Trigger {
  type Error = UserError;

  fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
    Trigger::try_from(FifoTrigger::parse(value)?)
  }
}

#[cfg(test)]
mod tests {

  mod into_trigger {
    use crate::client::{FifoTrigger, Trigger};
    use big_s::S;

    #[test]
    fn test_all() {
      let fifo_trigger = FifoTrigger {
        command: S("testAll"),
        ..FifoTrigger::default()
      };
      let have = Trigger::try_from(fifo_trigger).unwrap();
      let want = Trigger::TestAll;
      assert_eq!(have, want);
    }

    #[test]
    fn repeat_test() {
      let fifo_data = FifoTrigger {
        command: S("repeatTest"),
        ..FifoTrigger::default()
      };
      let have = Trigger::try_from(fifo_data).unwrap();
      let want = Trigger::RepeatLastTest;
      assert_eq!(have, want);
    }

    mod custom_command {
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          command: S("customCommand"),
          run: Some(S("echo hello")),
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data).unwrap();
        let want = Trigger::CustomCommand { run: S("echo hello") };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_run() {
        let fifo_data = FifoTrigger {
          command: S("customCommand"),
          run: None,
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }
    }

    mod test_file {
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          command: S("testFile"),
          file: Some(S("file.rs")),
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data).unwrap();
        let want = Trigger::TestFile { file: S("file.rs") };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_file() {
        let fifo_data = FifoTrigger {
          command: S("testFile"),
          file: None,
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }
    }

    mod test_function {
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          command: S("testFileLine"),
          file: Some(S("file.rs")),
          line: Some(2),
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data).unwrap();
        let want = Trigger::TestFileLine { file: S("file.rs"), line: 2 };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_file() {
        let fifo_data = FifoTrigger {
          command: S("testFileLine"),
          file: None,
          line: Some(2),
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }

      #[test]
      fn missing_line() {
        let fifo_data = FifoTrigger {
          command: S("testFileLine"),
          file: Some(S("file.rs")),
          line: None,
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }
    }
  }
}
