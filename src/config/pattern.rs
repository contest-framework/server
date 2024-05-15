use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::Trigger;

/// a pattern in the config file that matches incoming commands from the client
pub struct Pattern {
  command: Regex,
  pub file: Option<Regex>,
}

impl<'de> Deserialize<'de> for Pattern {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Inner {
      command: String,
      file: Option<String>,
    }
    let inner = Inner::deserialize(deserializer)?;
    let command_regex = Regex::new(&inner.command).map_err(|err| {
      serde::de::Error::custom(format!("Invalid regex for field \"command\": {}", err))
    })?;
    let file_regex = if let Some(file) = &inner.file {
      Some(Regex::new(file).map_err(|err| {
        serde::de::Error::custom(format!("Invalide regex for field \"file\": {}", err))
      })?)
    } else {
      None
    };
    Ok(Pattern {
      command: command_regex,
      file: file_regex,
    })
  }
}

impl Pattern {
  pub fn matches_client_trigger(&self, from_client: &Trigger) -> crate::Result<bool> {
    if self.command.is_match(&from_client.command) {
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

#[cfg(test)]
mod tests {

  mod matches_client_trigger {
    use super::super::Pattern;
    use crate::Trigger;
    use big_s::S;
    use regex::Regex;

    #[test]
    fn matching() {
      let pattern = Pattern {
        command: Regex::new("testFunction").unwrap(),
        file: Some(Regex::new("\\.rs$").unwrap()),
      };
      let give = Trigger::TestFileLine {
        file: S("subdir1/subdir2/bar.rs"),
        line: 12,
      };
      assert!(pattern.matches_client_trigger(&give).unwrap());
    }

    #[test]
    fn mismatching_command() {
      let pattern = Pattern {
        command: "testFunction".into(),
        file: Some("filename".into()),
        line: None,
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("filename".into()),
        line: None,
      };
      assert!(!pattern.matches_client_trigger(&give).unwrap());
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
