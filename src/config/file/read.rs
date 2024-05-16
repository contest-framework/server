use super::super::Configuration;
use super::{FileConfiguration, PATH};
use crate::{Result, UserError};
use std::fs::File;
use std::io;

pub fn read() -> Result<Configuration> {
  let file = File::open(PATH).map_err(|err| match err.kind() {
    io::ErrorKind::NotFound => UserError::ConfigFileNotFound {},
    _ => UserError::ConfigFileError {
      err: err.to_string(),
    },
  })?;
  let file_data: FileConfiguration =
    serde_json::from_reader(file).map_err(|err| UserError::ConfigFileInvalidContent {
      err: err.to_string(),
    })?;
  file_data.into_domain()
}
