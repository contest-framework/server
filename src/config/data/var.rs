#[derive(Debug)]
pub struct Var {
  pub name: String,
  pub source: VarSource,
  pub filter: regex::Regex,
}

fn calculate_var(var: &Var, values: &HashMap<&str, String>) -> Result<String> {
  match var.source {
    VarSource::File => filter(values.get("file").unwrap(), &var.filter),
    VarSource::Line => filter(values.get("line").unwrap(), &var.filter),
    VarSource::CurrentOrAboveLineContent => {
      let file_name = values.get("file").unwrap();
      let file_content = fs::read_to_string(file_name).unwrap();
      let lines: Vec<&str> = file_content.split('\n').collect();
      let re = Regex::new(&var.filter).unwrap();
      let Some(original_line) = values.get("line") else {
        return Err(UserError::MissingLineFieldInCurrentOrAboveLineContent);
      };
      let original_line = original_line.parse().unwrap();
      let mut line = original_line;
      while line > 0 {
        line -= 1;
        let line_text: String = lines.get(line as usize).unwrap().to_string();
        let captures = re.captures(&line_text);
        if captures.is_none() {
          // no match on this line --> try the one above
          continue;
        }
        let captures = captures.unwrap();
        if captures.len() > 2 {
          return Err(UserError::TriggerTooManyCaptures {
            count: captures.len(),
            regex: var.filter.to_owned(),
            line: line_text,
          });
        }
        return Ok(captures.get(1).unwrap().as_str().to_owned());
      }
      Err(UserError::TriggerRegexNotFound {
        regex: var.filter.to_owned(),
        filename: file_name.to_string(),
        line: original_line,
      })
    }
  }
}

fn filter(text: &str, filter: &str) -> Result<String> {
  let re = Regex::new(filter).unwrap();
  let captures = re.captures(text).unwrap();
  if captures.len() != 2 {
    return Err(UserError::TriggerTooManyCaptures {
      count: captures.len(),
      regex: filter.to_owned(),
      line: text.to_owned(),
    });
  }
  return Ok(captures.get(1).unwrap().as_str().to_owned());
}

fn replace(text: &str, placeholder: &str, replacement: &str) -> String {
  Regex::new(&format!("\\{{\\{{\\s*{}\\s*\\}}\\}}", placeholder))
    .unwrap()
    .replace_all(text, regex::NoExpand(replacement))
    .to_string()
}

#[cfg(test)]
mod tests {

  #[cfg(test)]
  mod replace {
    use super::super::replace;

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
}
