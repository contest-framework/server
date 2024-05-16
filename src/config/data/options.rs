use super::{AfterRun, BeforeRun};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Options {
  pub before_run: BeforeRun,
  pub after_run: AfterRun,
}
