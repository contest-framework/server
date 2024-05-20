use super::super::Configuration;
use super::{FileConfiguration, JSON5_PATH, JSON_PATH};
use crate::{Result, UserError};
use std::{fs, io};

pub fn read() -> Result<Configuration> {
  let file_content = match fs::read_to_string(JSON_PATH) {
    Ok(file) => file,
    Err(err) if err.kind() == io::ErrorKind::NotFound => match fs::read_to_string(JSON5_PATH) {
      Ok(file) => file,
      Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Configuration::default()),
      Err(err) => {
        return Err(UserError::ConfigFileError {
          err: err.to_string(),
        })
      }
    },
    Err(err) => {
      return Err(UserError::ConfigFileError {
        err: err.to_string(),
      })
    }
  };
  let file_data: FileConfiguration =
    json5::from_str(&file_content).map_err(|err| UserError::ConfigFileInvalidContent {
      err: err.to_string(),
    })?;
  file_data.into_domain()
}
