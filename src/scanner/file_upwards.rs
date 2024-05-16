use super::string_upwards;
use crate::{Result, UserError};
use regex::Regex;
use std::fs;

/// provides the first match of the given `Regex`
/// in the content of the file with the given path
/// starting at the given index and scanning towards the file beginning
pub fn file_upwards(file_path: &str, re: &Regex, index: u32) -> Result<String> {
  let file_content = fs::read_to_string(file_path).unwrap();
  let Some(result) = string_upwards(&file_content, re, index)? else {
    return Err(UserError::TriggerRegexNotFound {
      regex: re.to_string(),
      filename: file_path.to_owned(),
      line: index,
    });
  };
  Ok(result)
}
