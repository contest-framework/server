use super::string_upwards;
use crate::{Result, UserError};
use regex::Regex;
use std::fs;

pub fn file_upwards(file_name: &str, re: &Regex, index: u32) -> Result<String> {
  let file_content = fs::read_to_string(file_name).unwrap();
  let Some(result) = string_upwards(&file_content, re, index)? else {
    return Err(UserError::TriggerRegexNotFound {
      regex: re.to_string(),
      filename: file_name.to_owned(),
      line: index,
    });
  };
  Ok(result)
}
