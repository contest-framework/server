use super::{FileAfterRun, FileBeforeRun};
use crate::config::Options;
use serde::Deserialize;

/// low-level, unvalidated `Options` data exactly how it is stored in the config file
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileOptions {
  before_run: Option<FileBeforeRun>,
  after_run: Option<FileAfterRun>,
}

impl FileOptions {
  pub fn into_domain(self) -> Options {
    Options {
      before_run: self.before_run.unwrap_or_default().into_domain(),
      after_run: self.after_run.unwrap_or_default().into_domain(),
    }
  }
}

#[cfg(test)]
mod tests {

  mod to_domain {
    use super::super::FileOptions;
    use crate::config::file::{FileAfterRun, FileBeforeRun};
    use crate::config::{AfterRun, BeforeRun, Options};

    #[test]
    fn empty() {
      let file_options = FileOptions {
        before_run: None,
        after_run: None,
      };
      let have = file_options.into_domain();
      let want = Options {
        before_run: BeforeRun {
          clear_screen: false,
          newlines: 0,
        },
        after_run: AfterRun {
          newlines: 0,
          indicator_lines: 0,
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
        }),
      };
      let have = file_options.into_domain();
      let want = Options {
        before_run: BeforeRun {
          clear_screen: true,
          newlines: 2,
        },
        after_run: AfterRun {
          newlines: 4,
          indicator_lines: 6,
        },
      };
      assert_eq!(have, want);
    }
  }
}
