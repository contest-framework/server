//! A very simple templating engine that just replaces placeholders in the string template.

pub fn replace(text: &str, placeholder: &str, replacement: &str) -> String {
  Regex::new(&format!("\\{{\\{{\\s*{}\\s*\\}}\\}}", placeholder))
    .unwrap()
    .replace_all(text, regex::NoExpand(replacement))
    .to_string()
}
