pub fn replace(text: &str, placeholder: &str, replacement: &str) -> String {
  super::regex(placeholder)
    .replace_all(text, regex::NoExpand(replacement))
    .to_string()
}

#[cfg(test)]
mod tests {
  use super::replace;

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
