use crate::{Result, UserError};
use serde::Deserialize;

/// The `Trigger` data as it comes in through the FIFO.
#[derive(Deserialize, Debug, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FifoTrigger {
  pub command: String,
  pub file: Option<String>,
  pub line: Option<usize>,
  pub run: Option<String>,
}

impl FifoTrigger {
  pub fn parse(line: &str) -> Result<Self> {
    let result: Self = json5::from_str(line).map_err(|err| UserError::InvalidTrigger {
      source: line.to_owned(),
      err: err.to_string(),
    })?;
    result.validate(line)?;
    Ok(result)
  }

  pub fn validate(&self, source: &str) -> Result<()> {
    let command = self.command.to_ascii_lowercase();
    if command == "testall" {
      return Ok(());
    }
    if command == "customcommand" {
      if self.run.is_some() {
        return Ok(());
      }
      return Err(UserError::InvalidTrigger {
        source: source.into(),
        err: r#"trigger "customCommand" is missing field "run"."#.into(),
      });
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
          });
        }
        (false, true) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testFileLine" is missing field "file""#.into(),
          });
        }
        (false, false) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testFileLine" is missing fields "file" and "line""#.into(),
          });
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

    mod custom_command {
      use crate::client::FifoTrigger;
      use big_s::S;

      #[test]
      fn valid() {
        let give = r#"{ "command": "customCommand", "run": "echo hello" }"#;
        let have = FifoTrigger::parse(give).unwrap();
        let want = FifoTrigger {
          command: S("customCommand"),
          run: Some(S("echo hello")),
          ..FifoTrigger::default()
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_run() {
        let give = r#"{ "command": "customCommand" }"#;
        let have = FifoTrigger::parse(give);
        assert!(have.is_err());
      }
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
        let give = r#"{ "command": "testFileLine", "file": "foo.rs", "line": 12 }"#;
        let have = FifoTrigger::parse(give).unwrap();
        let want = FifoTrigger {
          command: S("testFileLine"),
          file: Some(S("foo.rs")),
          line: Some(12),
          ..FifoTrigger::default()
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_file() {
        let give = r#"{ "command": "testFileLine", "line": 12 }"#;
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
      let give = r#"{ "command": "testFile", "file": "foo.rs", "other": 12 }"#;
      let have = FifoTrigger::parse(give);
      assert!(have.is_err());
    }

    #[test]
    fn invalid_json() -> Result<(), String> {
      let give = "{\"filename}";
      match FifoTrigger::parse(give) {
        Err(UserError::InvalidTrigger { source, err }) => {
          assert_eq!(source, give.to_owned());
          assert_eq!(err, S(" --> 1:12\n  |\n1 | {\"filename}\n  |            ^---\n  |\n  = expected char_literal"));
          Ok(())
        }
        Err(_) => Err(S("unexpected UserError")),
        Ok(_) => Err(S("this should not have worked")),
      }
    }
  }
}
