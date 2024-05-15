use super::super::Configuration;
use super::{FileContent, PATH};
use crate::{Result, UserError};
use std::fs::File;
use std::io;

pub fn read() -> Result<Configuration> {
  let file = File::open(PATH).map_err(|err| match err.kind() {
    io::ErrorKind::NotFound => UserError::ConfigFileNotFound {},
    _ => UserError::ConfigFileError { err },
  })?;
  let file_data: FileContent =
    serde_json::from_reader(file).map_err(|err| UserError::ConfigFileInvalidContent { err })?;
  file_data.to_configuration()
}
