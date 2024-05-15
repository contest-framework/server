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
    if self == &Pattern::TestAll && trigger == &Trigger::TestAll {
      return true;
    }
    if let Pattern::TestFile { files } = &self {
      if let Trigger::TestFile { file } = &trigger {
        return files.matches(file);
      }
    }
    if let Pattern::TestFileLine { files } = &self {
      if let Trigger::TestFileLine { file, line: _ } = &trigger {
        return files.matches(file);
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

      #[test]
      fn matches() {
        let config_trigger = config::Pattern::TestAll;
        let client_trigger = client::Trigger::TestAll;
        assert!(config_trigger.matches_trigger(&client_trigger));
      }
    }
  }
}
