use crate::config::AfterRun;
use serde::Deserialize;

/// low-level, unvalidated `AfterRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileAfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
  pub print_result: Option<bool>,
}

impl FileAfterRun {
  pub fn into_domain(self) -> AfterRun {
    let defaults = AfterRun::default();
    AfterRun {
      newlines: self.newlines.unwrap_or(defaults.newlines),
      indicator_lines: self.indicator_lines.unwrap_or(defaults.indicator_lines),
      print_result: self.print_result.unwrap_or(defaults.print_result),
    }
  }
}

#[cfg(test)]
mod tests {

  mod into_domain {
    use super::super::FileAfterRun;
    use crate::config::AfterRun;

    #[test]
    fn empty() {
      let file_after_run = FileAfterRun {
        newlines: None,
        indicator_lines: None,
        print_result: None,
      };
      let have = file_after_run.into_domain();
      let want = AfterRun {
        newlines: 0,
        indicator_lines: 0,
        print_result: true,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn with_content() {
      let file_after_run = FileAfterRun {
        newlines: Some(2),
        indicator_lines: Some(4),
        print_result: Some(false),
      };
      let have = file_after_run.into_domain();
      let want = AfterRun {
        newlines: 2,
        indicator_lines: 4,
        print_result: false,
      };
      assert_eq!(have, want);
    }
  }
}
