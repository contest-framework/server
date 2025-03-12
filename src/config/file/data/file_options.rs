use super::{FileAfterRun, FileBeforeRun};
use serde::Deserialize;

/// low-level, unvalidated `Options` data exactly how it is stored in the config file
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileOptions {
  pub before_run: Option<FileBeforeRun>,
  pub after_run: Option<FileAfterRun>,
}
