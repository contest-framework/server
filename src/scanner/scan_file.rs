use super::scan_string_upwards;
use crate::Result;
use crate::UserError;
use regex::Regex;
use std::fs;

pub fn scan_file_upwards(file_name: &str, re: &Regex, index: u32) -> Result<String> {
  let file_content = fs::read_to_string(file_name).unwrap();
  let Some(result) = scan_string_upwards(&file_content, &re, index)? else {
    return Err(UserError::TriggerRegexNotFound {
      regex: re.to_string(),
      filename: file_name.to_string(),
      line: index,
    });
  };
  Ok(result)
}
