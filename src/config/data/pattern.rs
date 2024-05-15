use crate::client::Trigger;

/// a pattern defined in the config file, describes conditions that match actions
#[derive(Debug, PartialEq)]
pub enum Pattern {
  TestAll,
  TestFile { files: glob::Pattern },
  TestFileLine { files: glob::Pattern },
}

impl Pattern {
  pub fn matches_trigger(&self, trigger: &Trigger) -> bool {
    match self {
      Pattern::TestAll => return trigger == &Trigger::TestAll,
      Pattern::TestFile { files } => {
        if let Trigger::TestFile { file } = &trigger {
          return files.matches(file);
        }
      }
      Pattern::TestFileLine { files } => {
        if let Trigger::TestFileLine { file, line: _ } = &trigger {
          return files.matches(file);
        }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {

  mod matches_client_trigger {

    mod test_all {
      use crate::client;
      use crate::config;
      use big_s::S;

      #[test]
      fn matches() {
        let config_trigger = config::Pattern::TestAll;
        let client_trigger = client::Trigger::TestAll;
        assert!(config_trigger.matches_trigger(&client_trigger));
      }

      #[test]
      fn mismatch() {
        let config_trigger = config::Pattern::TestAll;
        let client_trigger = client::Trigger::TestFile { file: S("file") };
        assert!(config_trigger.matches_trigger(&client_trigger));
      }
    }

    mod test_file {
      use crate::client;
      use crate::config;
      use big_s::S;

      #[test]
      fn matches() {
        let config_trigger = config::Pattern::TestFile {
          files: glob::Pattern::new("*.rs").unwrap(),
        };
        let client_trigger = client::Trigger::TestFile { file: S("foo.rs") };
        assert!(config_trigger.matches_trigger(&client_trigger));
      }

      #[test]
      fn mismatching_file() {
        let config_trigger = config::Pattern::TestFile {
          files: glob::Pattern::new("*.rs").unwrap(),
        };
        let client_trigger = client::Trigger::TestFile {
          file: S("mismatch.go"),
        };
        assert!(!config_trigger.matches_trigger(&client_trigger));
      }

      #[test]
      fn mismatching_type() {
        let config_trigger = config::Pattern::TestFile {
          files: glob::Pattern::new("*.rs").unwrap(),
        };
        let client_trigger = client::Trigger::TestAll;
        assert!(!config_trigger.matches_trigger(&client_trigger));
      }
    }

    mod test_file_line {
      use crate::client;
      use crate::config;
      use big_s::S;

      #[test]
      fn matches() {
        let config_trigger = config::Pattern::TestFileLine {
          files: glob::Pattern::new("*.rs").unwrap(),
        };
        let client_trigger = client::Trigger::TestFileLine {
          file: S("foo.rs"),
          line: 3,
        };
        assert!(config_trigger.matches_trigger(&client_trigger));
      }

      #[test]
      fn mismatching_file() {
        let config_trigger = config::Pattern::TestFileLine {
          files: glob::Pattern::new("*.rs").unwrap(),
        };
        let client_trigger = client::Trigger::TestFileLine {
          file: S("mismatch.go"),
          line: 3,
        };
        assert!(!config_trigger.matches_trigger(&client_trigger));
      }

      #[test]
      fn mismatching_type() {
        let config_trigger = config::Pattern::TestFileLine {
          files: glob::Pattern::new("*.rs").unwrap(),
        };
        let client_trigger = client::Trigger::TestAll;
        assert!(!config_trigger.matches_trigger(&client_trigger));
      }
    }
  }
}
