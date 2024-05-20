use crate::{Result, UserError};
use regex::Regex;

/// provides the first capture of the given regex in the given string
/// starting at the given line and scanning towards the beginning of the file
pub fn string_upwards(text: &str, re: &Regex, mut index: u32) -> Result<Option<String>> {
  let lines: Vec<&str> = text.split('\n').collect();
  while index > 0 {
    let Some(line_text) = lines.get(index as usize) else {
      return Ok(None);
    };
    index -= 1;
    let Some(captures) = re.captures(line_text) else {
      // no match on this line --> try the one above
      continue;
    };
    let Some(match_1) = captures.get(1) else {
      continue;
    };
    if captures.len() > 2 {
      // we should get only 2 captures: one for the entire string, and one for the capture
      return Err(UserError::TriggerTooManyCaptures {
        count: captures.len() as u32,
        regex: re.to_string(),
        line: (*line_text).to_owned(),
      });
    }
    return Ok(Some(match_1.as_str().to_owned()));
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
  println!("hello");
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
  println!("hello");
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
  println!("hello");
}
"#;
    let re = regex::Regex::new("fn (\\w+?)\\(").unwrap();
    let have = string_upwards(text, &re, 3).unwrap().unwrap();
    let want = "test_func";
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let text = "\
placeholder
";
    let re = regex::Regex::new("fn (\\w+?)\\(").unwrap();
    let have = string_upwards(text, &re, 1).unwrap();
    assert!(have.is_none());
  }
}
