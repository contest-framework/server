use super::Trigger;
use crate::{Result, UserError};
use serde::Deserialize;

/// The `Trigger` data as it comes in through the FIFO.
#[derive(Deserialize, Debug, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FifoTrigger {
  pub command: String,
  pub file: Option<String>,
  pub line: Option<String>,
  pub run: Option<String>,
}

impl FifoTrigger {
  pub fn parse(line: &str) -> Result<Self> {
    let result: Self = serde_json::from_str(line).map_err(|err| UserError::InvalidTrigger {
      source: line.to_owned(),
      err: err.to_string(),
    })?;
    result.validate(line)?;
    Ok(result)
  }

  pub fn into_trigger(self) -> Result<Trigger> {
    match self.command.to_ascii_lowercase().as_str() {
      "testall" => Ok(Trigger::TestAll),
      "repeattest" => Ok(Trigger::RepeatLastTest),
      "customCommand" => self.into_custom_command(),
      "testfile" => self.into_testfile(),
      "testfileline" => self.into_testfileline(),
      _ => Err(UserError::UnknownTrigger {
        source: self.command,
      }),
    }
  }

  fn into_custom_command(self) -> Result<Trigger> {
    match self.run {
      Some(run) => Ok(Trigger::CustomCommand { command: run }),
      None => Err(UserError::MissingRunInTrigger),
    }
  }

  fn into_testfile(self) -> Result<Trigger> {
    match self.file {
      Some(file) => Ok(Trigger::TestFile { file }),
      None => Err(UserError::MissingFileInTrigger),
    }
  }

  fn into_testfileline(self) -> Result<Trigger> {
    let Some(file) = self.file else {
      return Err(UserError::MissingFileInTrigger);
    };
    let Some(line) = self.line else {
      return Err(UserError::MissingLineInTrigger);
    };
    Ok(Trigger::TestFileLine { file, line })
  }

  pub fn validate(&self, source: &str) -> Result<()> {
    let command = self.command.to_ascii_lowercase();
    if command == "testall" {
      return Ok(());
    }
    if command == "testfile" {
      if self.file.is_some() {
        return Ok(());
      }
      return Err(UserError::InvalidTrigger {
        source: source.into(),
        err: r#"trigger "testFile" is missing field "file"."#.into(),
      });
    }
    if command == "testfileline" {
      match (self.file.is_some(), self.line.is_some()) {
        (true, true) => return Ok(()),
        (true, false) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testFileLine" is missing field "line""#.into(),
          })
        }
        (false, true) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testFileLine" is missing field "file""#.into(),
          })
        }
        (false, false) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testFileLine" is missing fields "file" and "line""#.into(),
          })
        }
      }
    }
    if command == "repeattest" {
      return Ok(());
    }
    Err(UserError::InvalidTrigger {
      source: source.into(),
      err: "unknown command".into(),
    })
  }
}

#[cfg(test)]
mod tests {

  mod parse {
    use super::super::FifoTrigger;
    use crate::UserError;
    use big_s::S;

    #[test]
    fn test_all() {
      let give = r#"{ "command": "testAll" }"#;
      let have = FifoTrigger::parse(give).unwrap();
      let want = FifoTrigger {
        command: S("testAll"),
        ..FifoTrigger::default()
      };
      assert_eq!(have, want);
    }

    mod test_file {
      use crate::client::FifoTrigger;
      use big_s::S;

      #[test]
      fn valid() {
        let give = r#"{ "command": "testFile", "file": "foo.rs" }"#;
        let have = FifoTrigger::parse(give).unwrap();
        let want = FifoTrigger {
          command: S("testFile"),
          file: Some(S("foo.rs")),
          ..FifoTrigger::default()
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_filename() {
        let give = r#"{ "command": "testFile" }"#;
        let have = FifoTrigger::parse(give);
        assert!(have.is_err());
      }
    }

    mod test_function {
      use crate::client::FifoTrigger;
      use big_s::S;

      #[test]
      fn valid() {
        let give = r#"{ "command": "testFileLine", "file": "foo.rs", "line": "12" }"#;
        let have = FifoTrigger::parse(give).unwrap();
        let want = FifoTrigger {
          command: S("testFileLine"),
          file: Some(S("foo.rs")),
          line: Some(S("12")),
          run: None,
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_file() {
        let give = r#"{ "command": "testFileLine", "line": "12" }"#;
        let have = FifoTrigger::parse(give);
        assert!(have.is_err());
      }

      #[test]
      fn no_line() {
        let give = r#"{ "command": "testFileLine", "file": "foo.rs" }"#;
        let have = FifoTrigger::parse(give);
        assert!(have.is_err());
      }
    }

    #[test]
    fn repeat_test() {
      let give = r#"{ "command": "repeatTest" }"#;
      let have = FifoTrigger::parse(give).unwrap();
      let want = FifoTrigger {
        command: S("repeatTest"),
        ..FifoTrigger::default()
      };
      assert_eq!(have, want);
    }

    #[test]
    fn unknown_fields() {
      let give = r#"{ "command": "testFile", "file": "foo.rs", "other": "12" }"#;
      let have = FifoTrigger::parse(give);
      assert!(have.is_err());
    }

    #[test]
    fn invalid_json() -> Result<(), String> {
      let give = "{\"filename}";
      match FifoTrigger::parse(give) {
        Err(UserError::InvalidTrigger { source, err }) => {
          assert_eq!(source, give.to_owned());
          assert_eq!(err, S("EOF while parsing a string at line 1 column 11"));
          Ok(())
        }
        Err(_) => Err(S("unexpected UserError")),
        Ok(_) => Err(S("this should not have worked")),
      }
    }
  }

  mod into_trigger {
    use crate::client::{FifoTrigger, Trigger};
    use big_s::S;

    #[test]
    fn test_all() {
      let fifo_data = FifoTrigger {
        command: S("testAll"),
        file: None,
        line: None,
      };
      let have = fifo_data.into_trigger().unwrap();
      let want = Trigger::TestAll;
      assert_eq!(have, want);
    }

    #[test]
    fn repeat_test() {
      let fifo_data = FifoTrigger {
        command: S("repeatTest"),
        file: None,
        line: None,
      };
      let have = fifo_data.into_trigger().unwrap();
      let want = Trigger::RepeatLastTest;
      assert_eq!(have, want);
    }

    mod test_file {
      use crate::client::{FifoTrigger, Trigger};
      use big_s::S;

      #[test]
      fn valid() {
        let fifo_data = FifoTrigger {
          command: S("testFile"),
          file: Some(S("file.rs")),
          line: None,
        };
        let have = fifo_data.into_trigger().unwrap();
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
        let have = fifo_data.into_trigger();
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
          line: Some(S("2")),
        };
        let have = fifo_data.into_trigger().unwrap();
        let want = Trigger::TestFileLine {
          file: S("file.rs"),
          line: S("2"),
        };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_file() {
        let fifo_data = FifoTrigger {
          command: S("testFileLine"),
          file: None,
          line: Some(S("2")),
        };
        let have = fifo_data.into_trigger();
        assert!(have.is_err());
      }

      #[test]
      fn missing_line() {
        let fifo_data = FifoTrigger {
          command: S("testFileLine"),
          file: Some(S("file.rs")),
          line: None,
        };
        let have = fifo_data.into_trigger();
        assert!(have.is_err());
      }
    }
  }
}
