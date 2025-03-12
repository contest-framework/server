use super::{AfterRun, BeforeRun};
use crate::config::file::data::FileOptions;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Options {
  pub before_run: BeforeRun,
  pub after_run: AfterRun,
}

impl From<FileOptions> for Options {
  fn from(value: FileOptions) -> Self {
    Options {
      before_run: BeforeRun::from(value.before_run.unwrap_or_default()),
      after_run: AfterRun::from(value.after_run.unwrap_or_default()),
    }
  }
}

#[cfg(test)]
mod tests {

  mod into_domain {
    use super::super::FileOptions;
    use crate::config::file::data::{FileAfterRun, FileBeforeRun};
    use crate::config::{AfterRun, BeforeRun, Options};

    #[test]
    fn empty() {
      let file_options = FileOptions {
        before_run: None,
        after_run: None,
      };
      let have = Options::from(file_options);
      let want = Options {
        before_run: BeforeRun { clear_screen: false, newlines: 0 },
        after_run: AfterRun {
          newlines: 0,
          indicator_lines: 0,
          print_result: true,
        },
      };
      assert_eq!(have, want);
    }

    #[test]
    fn has_content() {
      let file_options = FileOptions {
        before_run: Some(FileBeforeRun {
          clear_screen: Some(true),
          newlines: Some(2),
        }),
        after_run: Some(FileAfterRun {
          newlines: Some(4),
          indicator_lines: Some(6),
          print_result: Some(false),
        }),
      };
      let have = Options::from(file_options);
      let want = Options {
        before_run: BeforeRun { clear_screen: true, newlines: 2 },
        after_run: AfterRun {
          newlines: 4,
          indicator_lines: 6,
          print_result: false,
        },
      };
      assert_eq!(have, want);
    }
  }
}
