use regex::Regex;

/// provides a regex that matches a placeholder with the given text
pub fn regex(placeholder: &str) -> regex::Regex {
  Regex::new(&format!("\\{{\\{{\\s*{}\\s*\\}}\\}}", placeholder)).unwrap()
}
