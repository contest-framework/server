use super::FileAction;
use super::FileOptions;
use crate::config::{Action, Configuration};
use crate::Result;
use serde::Deserialize;

/// The structure of the configuration file.
#[derive(Deserialize)]
pub struct FileConfiguration {
  actions: Vec<FileAction>,
  options: Option<FileOptions>,
}

impl FileConfiguration {
  pub fn to_domain(self) -> Result<Configuration> {
    let mut actions: Vec<Action> = Vec::with_capacity(self.actions.len());
    for json_action in self.actions {
      actions.push(json_action.to_domain()?);
    }
    Ok(Configuration {
      actions,
      options: self.options.unwrap_or_default().into_domain(),
    })
  }
}
