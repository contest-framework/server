use regex::Regex;

pub fn regex(placeholder: &str) -> regex::Regex {
  Regex::new(&format!("\\{{\\{{\\s*{}\\s*\\}}\\}}", placeholder)).unwrap()
}
