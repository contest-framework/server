//! commands received from the client (through the FIFO)

use crate::{Result, UserError};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Trigger {
  TestAll,
  TestFile { file: String },
  TestFileLine { file: String, line: i64 },
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

// TODO: rename to Trigger::parse
pub fn from_string(line: &str) -> Result<Trigger> {
  match serde_json::from_str(line) {
    Ok(trigger) => Ok(trigger),
    Err(err) => Err(UserError::InvalidTrigger {
      line: line.to_owned(),
      err: err.to_string(),
    }),
  }
}

#[cfg(test)]
mod tests {

  mod from_string {
    use super::super::{from_string, Trigger};
    use crate::UserError;
    use big_s::S;

    #[test]
    fn test_all() {
      let give = r#"{ "command": "testAll" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger::TestAll;
      assert_eq!(have, want);
    }

    #[test]
    fn filename() {
      let give = r#"{ "command": "testFile", "file": "foo.rs" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger::TestFile { file: S("foo.rs") };
      assert_eq!(have, want);
    }

    #[test]
    fn filename_line() {
      let give = r#"{ "command": "testFunction", "file": "foo.rs", "line": "12" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger::TestFileLine {
        file: S("foo.rs"),
        line: 12,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn extra_fields() {
      let give = r#"{ "command": "testFile", "file": "foo.rs", "other": "12" }"#;
      let have = from_string(give);
      assert!(have.is_err());
    }

    #[test]
    fn invalid_json() {
      let give = "{\"filename}";
      let have = from_string(give);
      let want = UserError::InvalidTrigger {
        line: give.into(),
        err: "EOF while parsing a string at line 1 column 11".into(),
      };
      match have {
        Ok(_) => panic!("this shouldn't work"),
        Err(err) => assert_eq!(err, want),
      }
    }
  }
}
