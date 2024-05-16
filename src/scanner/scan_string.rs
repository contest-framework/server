use crate::{Result, UserError};
use regex::Regex;

/// provides the first capture of the given regex in the given string starting at the given line and looking upwards
pub fn scan_string_upwards(text: &str, re: &Regex, mut index: u32) -> Result<Option<String>> {
  let lines: Vec<&str> = text.split('\n').collect();
  while index > 0 {
    index -= 1;
    let line_text = lines.get(index as usize).unwrap();
    let Some(captures) = re.captures(&line_text) else {
      // no match on this line --> try the one above
      continue;
    };
    if captures.len() > 2 {
      // we should get only 2 captures: one for the entire string, and one for the capture
      return Err(UserError::TriggerTooManyCaptures {
        count: captures.len(),
        regex: re.to_string(),
        line: line_text.to_string(),
      });
    }
    return Ok(Some(captures.get(1).unwrap().as_str().to_owned()));
  }
  Ok(None)
}
