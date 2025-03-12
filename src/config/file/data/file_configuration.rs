use super::{FileAction, FileOptions};
use serde::Deserialize;

/// low-level, unvalidated `Configuration` data exactly how it is stored in the config file
#[derive(Deserialize)]
pub struct FileConfiguration {
  pub actions: Vec<FileAction>,
  pub options: Option<FileOptions>,
}
