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
  Quit,
}

impl Display for Trigger {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Trigger::TestAll => f.write_str("test-all"),
      Trigger::TestFile { file } => write!(f, "test-file {file}"),
      Trigger::TestFileLine { file, line } => write!(f, "test-file-line {file}:{line}"),
      Trigger::CustomCommand { run } => write!(f, "custom-command {run}"),
      Trigger::RepeatLastTest => f.write_str("repeat-test"),
      Trigger::Quit => f.write_str("quit"),
    }
  }
}

impl TryFrom<FifoTrigger> for Trigger {
  type Error = UserError;

  fn try_from(fifo: FifoTrigger) -> std::result::Result<Self, Self::Error> {
    match fifo.data.command.to_ascii_lowercase().as_str() {
      "test-all" => Ok(Trigger::TestAll),
      "repeat-test" => Ok(Trigger::RepeatLastTest),
      "custom-command" => match fifo.data.run {
        Some(run) => Ok(Trigger::CustomCommand { run }),
        None => Err(UserError::MissingRunInTrigger { original: fifo.original_line }),
      },
      "test-file" => match fifo.data.file {
        Some(file) => Ok(Trigger::TestFile { file }),
        None => Err(UserError::MissingFileInTrigger { original: fifo.original_line }),
      },
      "test-file-line" => match (fifo.data.file, fifo.data.line) {
        (Some(file), Some(line)) => Ok(Trigger::TestFileLine { file, line }),
        (None, Some(_)) => Err(UserError::MissingFileInTrigger { original: fifo.original_line }),
        (Some(_), None) => Err(UserError::MissingLineInTrigger { original: fifo.original_line }),
        (None, None) => Err(UserError::MissingFileAndLineInTrigger { original: fifo.original_line }),
      },
      "quit" => Ok(Trigger::Quit),
      _ => Err(UserError::UnknownTrigger { source: fifo.data.command }),
    }
  }
}

impl TryFrom<String> for Trigger {
  type Error = UserError;

  fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
    Trigger::try_from(FifoTrigger::parse(value)?)
  }
}

#[cfg(test)]
mod tests {

  mod into_trigger {
    use crate::client::fifo_data::FifoTriggerData;
    use crate::client::{FifoTrigger, Trigger};
    use big_s::S;

    #[test]
    fn test_all() {
      let fifo_trigger = FifoTrigger {
        data: FifoTriggerData {
          command: S("test-all"),
          ..FifoTriggerData::default()
        },
        ..FifoTrigger::default()
      };
      let have = Trigger::try_from(fifo_trigger).unwrap();
      let want = Trigger::TestAll;
      assert_eq!(have, want);
    }

    #[test]
    fn repeat_test() {
      let fifo_data = FifoTrigger {
        data: FifoTriggerData {
          command: S("repeat-test"),
          ..FifoTriggerData::default()
        },
        ..FifoTrigger::default()
      };
      let have = Trigger::try_from(fifo_data).unwrap();
      let want = Trigger::RepeatLastTest;
      assert_eq!(have, want);
    }

    mod custom_command {
      use crate::client::fifo_data::FifoTriggerData;
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("custom-command"),
            run: Some(S("echo hello")),
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data).unwrap();
        let want = Trigger::CustomCommand { run: S("echo hello") };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_run() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("custom-command"),
            run: None,
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }
    }

    mod test_file {
      use crate::client::fifo_data::FifoTriggerData;
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("test-file"),
            file: Some(S("file.rs")),
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data).unwrap();
        let want = Trigger::TestFile { file: S("file.rs") };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_file() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("test-file"),
            file: None,
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }
    }

    mod test_function {
      use crate::client::fifo_data::FifoTriggerData;
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("test-file-line"),
            file: Some(S("file.rs")),
            line: Some(2),
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data).unwrap();
        let want = Trigger::TestFileLine { file: S("file.rs"), line: 2 };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_file() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("test-file-line"),
            file: None,
            line: Some(2),
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }

      #[test]
      fn missing_line() {
        let fifo_data = FifoTrigger {
          data: FifoTriggerData {
            command: S("test-file-line"),
            file: Some(S("file.rs")),
            line: None,
            ..FifoTriggerData::default()
          },
          ..FifoTrigger::default()
        };
        let have = Trigger::try_from(fifo_data);
        assert!(have.is_err());
      }
    }
  }
}
