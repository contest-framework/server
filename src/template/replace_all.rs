use super::replace;
use crate::Result;
use ahash::AHashMap;

pub fn replace_all(text: &str, replacements: &AHashMap<&str, String>) -> Result<String> {
  let mut result = text.to_owned();
  for (placeholder, replacement) in replacements {
    result = replace(&result, placeholder, replacement)?;
  }
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::replace_all;
  use ahash::AHashMap;
  use big_s::S;

  #[test]
  fn normal() {
    let mut replacements = AHashMap::new();
    replacements.insert("foo", S("bar"));
    let give = "a skeleton walks into a {{ foo }}";
    let have = replace_all(give, &replacements).unwrap();
    let want = "a skeleton walks into a bar";
    assert_eq!(have, want);
  }

  #[test]
  fn no_placeholders() {
    let replacements = AHashMap::new();
    let give = "a skeleton walks into a {{ foo }}";
    let have = replace_all(give, &replacements).unwrap();
    let want = "a skeleton walks into a {{ foo }}";
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let mut replacements = AHashMap::new();
    replacements.insert("foo", S("bar"));
    let give = "a skeleton walks into a {{ other }}";
    let have = replace_all(give, &replacements).unwrap();
    let want = "a skeleton walks into a {{ other }}";
    assert_eq!(have, want);
  }
}
