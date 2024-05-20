use super::VarSource;
use crate::{scanner, Result, UserError};
use ahash::AHashMap;
use regex::Regex;

#[derive(Debug)]
pub struct Var {
  pub name: String,
  pub source: VarSource,
  pub filter: regex::Regex,
}

impl Var {
  pub fn calculate_var(&self, values: &AHashMap<&str, String>) -> Result<String> {
    match self.source {
      VarSource::File => {
        let Some(filename) = values.get("file") else {
          return Err(UserError::FileNameNotAvailable);
        };
        filter(filename, &self.filter)
      }
      VarSource::Line => {
        let Some(line) = values.get("line") else {
          return Err(UserError::LineNotAvailable);
        };
        filter(line, &self.filter)
      }
      VarSource::CurrentOrAboveLineContent => {
        let Some(filename) = values.get("file") else {
          return Err(UserError::FileNameNotAvailable);
        };
        let Some(original_line) = values.get("line") else {
          return Err(UserError::LineNotAvailable);
        };
        let original_line = original_line
          .parse()
          .map_err(|_| UserError::LineIsNotANumber {
            line: original_line.to_owned(),
          })?;
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
  let Some(captures) = filter.captures(text) else {
    return Ok(String::new());
  };
  let Some(result) = captures.get(1) else {
    return Ok(String::new());
  };
  if captures.len() > 2 {
    return Err(UserError::TriggerTooManyCaptures {
      count: captures.len(),
      regex: filter.to_string(),
      line: text.to_owned(),
    });
  }
  return Ok(result.as_str().to_owned());
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
