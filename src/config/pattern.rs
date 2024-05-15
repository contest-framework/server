use super::Var;
use crate::{Trigger, UserError};

/// a pattern in the config file that matches incoming commands from the client
pub enum Pattern {
  TestAll {
    run: String,
  },
  TestFile {
    files: glob::Pattern,
    run: String,
    vars: Vec<Var>,
  },
  TestFunction {
    files: glob::Pattern,
    line: i64,
    run: String,
    vars: Vec<Var>,
  },
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
