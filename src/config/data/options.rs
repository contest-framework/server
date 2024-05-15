use super::{AfterRun, BeforeRun};

#[derive(Debug, Eq, PartialEq)]
pub struct Options {
  pub before_run: BeforeRun,
  pub after_run: AfterRun,
}

impl Options {
  pub fn defaults() -> Options {
    Options {
      before_run: BeforeRun {
        clear_screen: false,
        newlines: 0,
      },
      after_run: AfterRun {
        newlines: 0,
        indicator_lines: 3,
      },
    }
  }
}
