use super::replace;
use std::collections::HashMap;

pub fn replace_all(text: String, replacements: &HashMap<&str, String>) -> String {
  let mut result = text;
  for (placeholder, replacement) in replacements {
    result = replace(&result, placeholder, replacement);
  }
  result
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
    let give = "my name is {{ foo }}";
    let have = replace_all(give.into(), &replacements);
    let want = "my name is bar";
    assert_eq!(have, want);
  }

  #[test]
  fn no_placeholders() {
    let replacements = HashMap::new();
    let give = "my name is {{ foo }}";
    let have = replace_all(give.into(), &replacements);
    let want = "my name is {{ foo }}";
    assert_eq!(have, want);
  }

  #[test]
  fn no_match() {
    let replacements = hashmap! {
      "foo" => S("bar"),
    };
    let give = "my name is {{ other }}";
    let have = replace_all(give.into(), &replacements);
    let want = "my name is {{ other }}";
    assert_eq!(have, want);
  }
}
