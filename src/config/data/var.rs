use regex::Regex;

use super::VarSource;
use crate::{Result, UserError};
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct Var {
  pub name: String,
  pub source: VarSource,
  pub filter: regex::Regex,
}

impl Var {
  fn calculate_var(&self, values: &HashMap<&str, String>) -> Result<String> {
    match self.source {
      VarSource::File => filter(values.get("file").unwrap(), &self.filter),
      VarSource::Line => filter(values.get("line").unwrap(), &self.filter),
      VarSource::CurrentOrAboveLineContent => {
        // TODO: extract this logic into a dedicated module for better testing
        let file_name = values.get("file").unwrap();
        let file_content = fs::read_to_string(file_name).unwrap();
        let lines: Vec<&str> = file_content.split('\n').collect();
        let Some(original_line) = values.get("line") else {
          return Err(UserError::MissingLineFieldInCurrentOrAboveLineContent);
        };
        let original_line = original_line.parse().unwrap();
        let mut line = original_line;
        while line > 0 {
          line -= 1;
          let line_text: String = lines.get(line as usize).unwrap().to_string();
          let captures = self.filter.captures(&line_text);
          if captures.is_none() {
            // no match on this line --> try the one above
            continue;
          }
          let captures = captures.unwrap();
          if captures.len() > 2 {
            return Err(UserError::TriggerTooManyCaptures {
              count: captures.len(),
              regex: self.filter.to_string(),
              line: line_text,
            });
          }
          return Ok(captures.get(1).unwrap().as_str().to_owned());
        }
        Err(UserError::TriggerRegexNotFound {
          regex: self.filter.to_string(),
          filename: file_name.to_string(),
          line: original_line,
        })
      }
    }
  }
}

impl PartialEq for Var {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
      && self.source == other.source
      && self.filter.to_string() == other.filter.to_string()
  }
}

fn filter(text: &str, filter: &Regex) -> Result<String> {
  let captures = filter.captures(text).unwrap();
  if captures.len() != 2 {
    return Err(UserError::TriggerTooManyCaptures {
      count: captures.len(),
      regex: filter.to_string(),
      line: text.to_owned(),
    });
  }
  return Ok(captures.get(1).unwrap().as_str().to_owned());
}

fn replace(text: &str, placeholder: &str, replacement: &str) -> String {
  Regex::new(&format!("\\{{\\{{\\s*{}\\s*\\}}\\}}", placeholder))
    .unwrap()
    .replace_all(text, regex::NoExpand(replacement))
    .to_string()
}

#[cfg(test)]
mod tests {

  mod eq {
    use super::super::{Var, VarSource};
    use big_s::S;

    #[test]
    fn equal() {
      let regex = "fn (.*) \\{";
      let left = Var {
        name: S("name"),
        source: VarSource::File,
        filter: regex::Regex::new(&regex).unwrap(),
      };
      let right = Var {
        name: S("name"),
        source: VarSource::File,
        filter: regex::Regex::new(&regex).unwrap(),
      };
      assert_eq!(left, right);
    }

    #[test]
    fn not_equal() {
      let left = Var {
        name: S("name"),
        source: VarSource::File,
        filter: regex::Regex::new("left regex").unwrap(),
      };
      let right = Var {
        name: S("name"),
        source: VarSource::File,
        filter: regex::Regex::new("right regex").unwrap(),
      };
      assert_ne!(left, right);
    }
  }

  #[cfg(test)]
  mod replace {
    use super::super::replace;

    #[test]
    fn tight_placeholder() {
      let have = replace("hello {{world}}", "world", "universe");
      assert_eq!(have, "hello universe");
    }

    #[test]
    fn loose_placeholder() {
      let have = replace("hello {{ world }}", "world", "universe");
      assert_eq!(have, "hello universe");
    }

    #[test]
    fn multiple_placeholders() {
      let have = replace("{{ hello }} {{ hello }}", "hello", "bye");
      assert_eq!(have, "bye bye");
    }
  }
}
