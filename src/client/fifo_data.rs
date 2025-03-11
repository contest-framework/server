use crate::{Result, UserError};
use serde::Deserialize;

/// The `Trigger` data as it comes in through the FIFO.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct FifoTrigger {
  pub data: FifoTriggerData,
  pub original_line: String,
}

#[derive(Deserialize, Debug, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FifoTriggerData {
  pub command: String,
  pub file: Option<String>,
  pub line: Option<usize>,
  pub run: Option<String>,
}

impl FifoTrigger {
  pub fn parse(line: String) -> Result<Self> {
    let data: FifoTriggerData = match json5::from_str(&line) {
      Ok(result) => result,
      Err(err) => {
        return Err(UserError::InvalidTrigger {
          source: line,
          err: err.to_string(),
        });
      }
    };
    Ok(FifoTrigger { data, original_line: line })
  }
}

#[cfg(test)]
mod tests {

  mod parse {
    use super::super::FifoTrigger;
    use crate::UserError;
    use crate::client::fifo_data::FifoTriggerData;
    use big_s::S;

    #[test]
    fn test_all() {
      let give = S(r#"{ "command": "testAll" }"#);
      let have = FifoTrigger::parse(give.clone()).unwrap();
      let want = FifoTrigger {
        data: FifoTriggerData {
          command: S("testAll"),
          ..FifoTriggerData::default()
        },
        original_line: give,
      };
      assert_eq!(have, want);
    }

    mod custom_command {
      use crate::client::FifoTrigger;
      use crate::client::fifo_data::FifoTriggerData;
      use big_s::S;

      #[test]
      fn valid() {
        let give = S(r#"{ "command": "customCommand", "run": "echo hello" }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("customCommand"),
            run: Some(S("echo hello")),
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_run() {
        let give = S(r#"{ "command": "customCommand" }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("customCommand"),
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }
    }

    mod test_file {
      use crate::client::FifoTrigger;
      use crate::client::fifo_data::FifoTriggerData;
      use big_s::S;

      #[test]
      fn valid() {
        let give = S(r#"{ "command": "testFile", "file": "foo.rs" }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("testFile"),
            file: Some(S("foo.rs")),
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_filename() {
        let give = S(r#"{ "command": "testFile" }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("testFile"),
            file: None,
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }
    }

    mod test_function {
      use crate::client::FifoTrigger;
      use crate::client::fifo_data::FifoTriggerData;
      use big_s::S;

      #[test]
      fn valid() {
        let give = S(r#"{ "command": "testFileLine", "file": "foo.rs", "line": 12 }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("testFileLine"),
            file: Some(S("foo.rs")),
            line: Some(12),
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_file() {
        let give = S(r#"{ "command": "testFileLine", "line": 12 }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("testFileLine"),
            file: None,
            line: Some(12),
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }

      #[test]
      fn no_line() {
        let give = S(r#"{ "command": "testFileLine", "file": "foo.rs" }"#);
        let have = FifoTrigger::parse(give.clone()).unwrap();
        let want = FifoTrigger {
          data: FifoTriggerData {
            command: S("testFileLine"),
            file: Some(S("foo.rs")),
            line: None,
            ..FifoTriggerData::default()
          },
          original_line: give,
        };
        assert_eq!(have, want);
      }
    }

    #[test]
    fn repeat_test() {
      let give = S(r#"{ "command": "repeatTest" }"#);
      let have = FifoTrigger::parse(give.clone()).unwrap();
      let want = FifoTrigger {
        data: FifoTriggerData {
          command: S("repeatTest"),
          ..FifoTriggerData::default()
        },
        original_line: give,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn unknown_fields() {
      let give = S(r#"{ "command": "testFile", "file": "foo.rs", "other": 12 }"#);
      let have = FifoTrigger::parse(give);
      assert!(have.is_err());
    }

    #[test]
    fn invalid_json() -> Result<(), String> {
      let give = S("{\"filename}");
      match FifoTrigger::parse(give.clone()) {
        Err(UserError::InvalidTrigger { source, err }) => {
          assert_eq!(source, give);
          assert_eq!(err, S(" --> 1:12\n  |\n1 | {\"filename}\n  |            ^---\n  |\n  = expected char_literal"));
          Ok(())
        }
        Err(_) => Err(S("unexpected UserError")),
        Ok(_) => Err(S("this should not have worked")),
      }
    }
  }
}
