use crate::client::Trigger;
use std::fmt::Display;

/// a pattern defined in the config file, describes conditions that match actions
#[derive(Debug, Eq, PartialEq)]
pub enum Pattern {
  TestAll,
  TestFile { files: glob::Pattern },
  TestFileLine { files: glob::Pattern },
}

impl Pattern {
  /// indicates whether this `Pattern` matches the given `Trigger` received via the FIFO
  #[must_use]
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

impl Display for Pattern {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Pattern::TestAll => f.write_str("TestAll"),
      Pattern::TestFile { files } => write!(f, "TestFile {files}"),
      Pattern::TestFileLine { files } => write!(f, "TestFileLine {files}"),
    }
  }
}

#[cfg(test)]
mod tests {

  mod matches_client_trigger {

    mod test_all {
      use crate::{client, config};
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
        assert!(!config_trigger.matches_trigger(&client_trigger));
      }
    }

    mod test_file {
      use crate::{client, config};
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
      use crate::{client, config};
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
