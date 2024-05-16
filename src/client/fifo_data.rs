use crate::{Result, UserError};
use serde::Deserialize;

use super::Trigger;

/// The `Trigger` data as it comes in through the FIFO.
#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FifoTrigger {
  pub command: String,
  pub file: Option<String>,
  pub line: Option<String>,
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
    let command = self.command.to_ascii_lowercase();
    if command == "testall" {
      return Ok(Trigger::TestAll);
    }
    let Some(file) = self.file else {
      return Err(UserError::MissingFileInTrigger);
    };
    if command == "testfile" {
      return Ok(Trigger::TestFile { file });
    }
    let Some(line) = self.line else {
      return Err(UserError::MissingLineInTrigger);
    };
    if command == "testfunction" {
      return Ok(Trigger::TestFileLine { file, line });
    };
    Err(UserError::UnknownTrigger { line: self.command })
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
    if command == "testfunction" {
      match (self.file.is_some(), self.line.is_some()) {
        (true, true) => return Ok(()),
        (true, false) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testLine" is missing field "line""#.into(),
          })
        }
        (false, true) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testLine" is missing field "file""#.into(),
          })
        }
        (false, false) => {
          return Err(UserError::InvalidTrigger {
            source: source.into(),
            err: r#"trigger "testLine" is missing fields "file" and "line""#.into(),
          })
        }
      }
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
        file: None,
        line: None,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn filename() {
      let give = r#"{ "command": "testFile", "file": "foo.rs" }"#;
      let have = FifoTrigger::parse(give).unwrap();
      let want = FifoTrigger {
        command: S("testFile"),
        file: Some(S("foo.rs")),
        line: None,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn filename_line() {
      let give = r#"{ "command": "testFunction", "file": "foo.rs", "line": "12" }"#;
      let have = FifoTrigger::parse(give).unwrap();
      let want = FifoTrigger {
        command: S("testFunction"),
        file: Some(S("foo.rs")),
        line: Some(S("12")),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn extra_fields() {
      let give = r#"{ "command": "testFile", "file": "foo.rs", "other": "12" }"#;
      let have = FifoTrigger::parse(give);
      assert!(have.is_err());
    }

    #[test]
    fn invalid_json() {
      let give = "{\"filename}";
      let have = FifoTrigger::parse(give);
      let want = UserError::InvalidTrigger {
        source: give.into(),
        err: "EOF while parsing a string at line 1 column 11".into(),
      };
      match have {
        Ok(_) => panic!("this shouldn't work"),
        Err(err) => assert_eq!(err, want),
      }
    }
  }
}
