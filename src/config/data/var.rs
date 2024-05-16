use super::VarSource;
use crate::{scanner, Result, UserError};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Var {
  pub name: String,
  pub source: VarSource,
  pub filter: regex::Regex,
}

impl Var {
  pub fn calculate_var(&self, values: &HashMap<&str, String>) -> Result<String> {
    match self.source {
      VarSource::File => filter(values.get("file").unwrap(), &self.filter),
      VarSource::Line => filter(values.get("line").unwrap(), &self.filter),
      VarSource::CurrentOrAboveLineContent => {
        let filename = values.get("file").unwrap();
        let Some(original_line) = values.get("line") else {
          return Err(UserError::MissingLineFieldInCurrentOrAboveLineContent);
        };
        let original_line = original_line.parse().unwrap();
        scanner::file_upwards(filename, &self.filter, original_line)
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
        filter: regex::Regex::new(regex).unwrap(),
      };
      let right = Var {
        name: S("name"),
        source: VarSource::File,
        filter: regex::Regex::new(regex).unwrap(),
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
}
