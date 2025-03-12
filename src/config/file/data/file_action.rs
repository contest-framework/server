use super::FileVar;
use serde::Deserialize;

/// low-level, unvalidated `Action` data exactly how it is stored in the config file
#[derive(Deserialize)]
pub struct FileAction {
  pub r#type: String,
  pub files: Option<String>,
  pub run: String,
  pub vars: Option<Vec<FileVar>>,
}
