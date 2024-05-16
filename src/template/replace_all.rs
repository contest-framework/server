use super::replace;
use crate::Result;
use std::collections::HashMap;

pub fn replace_all(text: &str, replacements: &HashMap<&str, String>) -> Result<String> {
  let mut result = text.to_owned();
  for (placeholder, replacement) in replacements {
    result = replace(&result, placeholder, replacement)?;
  }
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::replace_all;
  use big_s::S;
  use maplit::hashmap;
  use std::collections::HashMap;

  #[test]
  fn normal() {
    let replacements = hashmap! {
      "foo" => S("bar"),
    };
    let give = "a skeleton walks into a {{ foo }}";
    let have = replace_all(give, &replacements).unwrap();
    let want = "a skeleton walks into a bar";
    assert_eq!(have, want);
  }

  #[test]
  fn no_placeholders() {
    let replacements = HashMap::new();
    let give = "a skeleton walks into a {{ foo }}";
    let have = replace_all(give, &replacements).unwrap();
    let want = "a skeleton walks into a {{ foo }}";
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let replacements = hashmap! {
      "foo" => S("bar"),
    };
    let give = "a skeleton walks into a {{ other }}";
    let have = replace_all(give, &replacements).unwrap();
    let want = "a skeleton walks into a {{ other }}";
    assert_eq!(have, want);
  }
}
