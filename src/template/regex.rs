use crate::{Result, UserError};
use regex::Regex;

/// provides a regex that matches a placeholder with the given text
pub fn regex(placeholder: &str) -> Result<regex::Regex> {
  let template = format!("\\{{\\{{\\s*{placeholder}\\s*\\}}\\}}");
  Regex::new(&template).map_err(|err| UserError::InvalidRegex {
    regex: template,
    err: err.to_string(),
  })
}
