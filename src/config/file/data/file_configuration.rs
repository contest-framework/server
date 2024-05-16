use super::{FileAction, FileOptions};
use crate::config::{Action, Configuration};
use crate::Result;
use serde::Deserialize;

/// low-level, unvalidated `Configuration` data exactly how it is stored in the config file
#[derive(Deserialize)]
pub struct FileConfiguration {
  actions: Vec<FileAction>,
  options: Option<FileOptions>,
}

impl FileConfiguration {
  pub fn into_domain(self) -> Result<Configuration> {
    let mut actions: Vec<Action> = Vec::with_capacity(self.actions.len());
    for json_action in self.actions {
      actions.push(json_action.into_domain()?);
    }
    Ok(Configuration {
      actions,
      options: self.options.unwrap_or_default().into_domain(),
    })
  }
}
