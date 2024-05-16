use crate::{Result, UserError};
use regex::Regex;

/// provides the first capture of the given regex in the given string starting at the given line and looking upwards
pub fn string_upwards(text: &str, re: &Regex, mut index: u32) -> Result<Option<String>> {
  let lines: Vec<&str> = text.split('\n').collect();
  while index > 0 {
    let line_text = lines.get(index as usize).unwrap();
    index -= 1;
    let Some(captures) = re.captures(line_text) else {
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

#[cfg(test)]
mod tests {
  use super::string_upwards;

  #[test]
  fn match_on_the_given_line() {
    let text = r#"\
//! a test module

pub fn test_func(param: &str) -> String {
  println!("a test function");
}
"#;
    let re = regex::Regex::new("fn (\\w+?)\\(").unwrap();
    let have = string_upwards(text, &re, 3).unwrap().unwrap();
    let want = "test_func";
    assert_eq!(have, want);
  }

  #[test]
  fn match_on_the_line_above() {
    let text = r#"\
//! a test module

pub fn test_func(param: &str) -> String {
  println!("a test function");
}
"#;
    let re = regex::Regex::new("fn (\\w+?)\\(").unwrap();
    let have = string_upwards(text, &re, 4).unwrap().unwrap();
    let want = "test_func";
    assert_eq!(have, want);
  }

  #[test]
  fn match_on_the_first_line() {
    let text = r#"\
pub fn test_func(param: &str) -> String {
  println!("a test function");
}
"#;
    let re = regex::Regex::new("fn (\\w+?)\\(").unwrap();
    let have = string_upwards(text, &re, 3).unwrap().unwrap();
    let want = "test_func";
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let text = r#"\
placeholder
"#;
    let re = regex::Regex::new("fn (\\w+?)\\(").unwrap();
    let have = string_upwards(text, &re, 1).unwrap();
    assert!(have.is_none());
  }
}
