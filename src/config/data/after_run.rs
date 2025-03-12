use crate::config::file::FileAfterRun;

#[derive(Debug, Eq, PartialEq)]
pub struct AfterRun {
  pub newlines: u8,
  pub indicator_lines: u8,
  pub print_result: bool,
}

impl Default for AfterRun {
  fn default() -> Self {
    Self {
      newlines: 0,
      indicator_lines: 0,
      print_result: true,
    }
  }
}

impl From<FileAfterRun> for AfterRun {
  fn from(value: FileAfterRun) -> Self {
    let defaults = AfterRun::default();
    AfterRun {
      newlines: value.newlines.unwrap_or(defaults.newlines),
      indicator_lines: value.indicator_lines.unwrap_or(defaults.indicator_lines),
      print_result: value.print_result.unwrap_or(defaults.print_result),
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
      let have = AfterRun::from(file_after_run);
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
      let have = AfterRun::from(file_after_run);
      let want = AfterRun {
        newlines: 2,
        indicator_lines: 4,
        print_result: false,
      };
      assert_eq!(have, want);
    }
  }
}
