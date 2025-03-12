use crate::config::VarSource;
use serde::Deserialize;

/// low-level, unvalidated `Var` data exactly how it is stored in the config file
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct FileVar {
  pub name: String,
  pub source: VarSource,
  pub filter: String,
}
