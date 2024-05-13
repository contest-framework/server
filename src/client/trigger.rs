//! commands received from the client (through the FIFO)

use crate::{Result, UserError};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
  pub command: String,
  pub file: Option<String>,
  pub line: Option<String>,
}

impl Trigger {
  pub fn matches_client_trigger(&self, from_client: &Trigger) -> Result<bool> {
    if self.command != from_client.command {
      return Ok(false);
    }
    if self.line.is_some() && from_client.line.is_none() {
      // config expects a line but client didn't send one --> no match
      return Ok(false);
    }
    if self.line.is_some() && from_client.line.is_some() {
      let self_line = &self.line.as_ref().unwrap();
      // TODO: pre-compute this pattern
      let pattern =
        glob::Pattern::new(self_line).map_err(|e| UserError::ConfigInvalidGlobPattern {
          pattern: self_line.to_string(),
          err: e.to_string(),
        })?;
      if !pattern.matches(from_client.line.as_ref().unwrap()) {
        return Ok(false);
      }
    }
    if self.file.is_some() && from_client.file.is_none() {
      // config expects a file but client didn't send one --> no match
      return Ok(false);
    }
    if self.file.is_some() && from_client.file.is_some() {
      let self_file = &self.file.as_ref().unwrap();
      // TODO: pre-compute this pattern
      let pattern =
        glob::Pattern::new(self_file).map_err(|e| UserError::ConfigInvalidGlobPattern {
          pattern: self_file.to_string(),
          err: e.to_string(),
        })?;
      if !pattern.matches(from_client.file.as_ref().unwrap()) {
        return Ok(false);
      }
    }
    Ok(true)
  }
}

impl Display for Trigger {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{ ")?;
    let mut parts: Vec<String> = Vec::new();
    parts.push(format!("\"command\": \"{}\"", self.command));
    if self.file.is_some() {
      parts.push(format!("\"file\": \"{}\"", self.file.as_ref().unwrap()));
    }
    if self.line.is_some() {
      parts.push(format!("\"line\": \"{}\"", self.line.as_ref().unwrap()));
    }
    write!(f, "{}", parts.join(", "))?;
    write!(f, " }}")
  }
}

pub fn from_string(line: &str) -> Result<Trigger> {
  match serde_json::from_str(line) {
    Ok(trigger) => Ok(trigger),
    Err(err) => Err(UserError::InvalidTrigger {
      line: line.to_owned(),
      err: err.to_string(),
    }),
  }
}

//
// ----------------------------------------------------------------------------
//

#[cfg(test)]
mod tests {

  mod from_string {

    use super::super::*;

    #[test]
    fn test_all() {
      let give = r#"{ "command": "testAll" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger {
        command: "testAll".into(),
        file: None,
        line: None,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn filename() {
      let give = r#"{ "command": "testFile", "file": "foo.rs" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger {
        command: "testFile".into(),
        file: Some("foo.rs".into()),
        line: None,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn filename_line() {
      let give = r#"{ "command": "testFunction", "file": "foo.rs", "line": "12" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger {
        command: "testFunction".into(),
        file: Some("foo.rs".into()),
        line: Some("12".into()),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn filename_extra_fields() {
      let give = r#"{ "command": "testFile", "file": "foo.rs", "other": "12" }"#;
      let have = from_string(give).unwrap();
      let want = Trigger {
        command: "testFile".into(),
        file: Some(String::from("foo.rs")),
        line: None,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn from_string_invalid_json() {
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

  mod matches_client_trigger {
    use super::super::*;

    #[test]
    fn matching() {
      let config = Trigger {
        command: "testFunction".into(),
        file: Some("**/*.rs".into()),
        line: Some("*".into()),
      };
      let give = Trigger {
        command: "testFunction".into(),
        file: Some("foo.rs".into()),
        line: Some("12".into()),
      };
      assert!(config.matches_client_trigger(&give).unwrap());
      let give = Trigger {
        command: "testFunction".into(),
        file: Some("subdir1/subdir2/bar.rs".into()),
        line: Some("12".into()),
      };
      assert!(config.matches_client_trigger(&give).unwrap());
    }

    #[test]
    fn mismatching_command() {
      let config = Trigger {
        command: "testFunction".into(),
        file: Some("filename".into()),
        line: None,
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("filename".into()),
        line: None,
      };
      assert!(!config.matches_client_trigger(&give).unwrap());
    }

    #[test]
    fn mismatching_file() {
      let config = Trigger {
        command: "testFunction".into(),
        file: Some("filename".into()),
        line: None,
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("otherfilename".into()),
        line: None,
      };
      assert!(!config.matches_client_trigger(&give).unwrap());
    }

    #[test]
    fn mismatching_line() {
      let config = Trigger {
        command: "testFunction".into(),
        file: Some("filename".into()),
        line: Some("*-*".into()),
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("filename".into()),
        line: Some("12".into()),
      };
      assert!(!config.matches_client_trigger(&give).unwrap());
    }

    #[test]
    fn missing_line() {
      let config = Trigger {
        command: "testFunction".into(),
        file: Some("filename".into()),
        line: Some("12".into()),
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("filename".into()),
        line: None,
      };
      assert!(!config.matches_client_trigger(&give).unwrap());
    }

    #[test]
    fn extra_line() {
      let config = Trigger {
        command: "testFunction".into(),
        file: Some("filename".into()),
        line: None,
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("filename".into()),
        line: Some("12".into()),
      };
      assert!(!config.matches_client_trigger(&give).unwrap());
    }
  }
}
