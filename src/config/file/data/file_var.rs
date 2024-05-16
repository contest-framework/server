use crate::config::{Var, VarSource};
use crate::{Result, UserError};
use serde::Deserialize;

/// low-level, unvalidated `Var` data exactly how it is stored in the config file
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct FileVar {
  pub name: String,
  pub source: VarSource,
  pub filter: String,
}

impl FileVar {
  pub fn into_domain(self) -> Result<Var> {
    let filter = regex::Regex::new(&self.filter).map_err(|err| UserError::InvalidRegex {
      regex: self.filter,
      err: err.to_string(),
    })?;
    Ok(Var {
      name: self.name,
      source: self.source,
      filter,
    })
  }
}
