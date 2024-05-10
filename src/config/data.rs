use crate::{Result, Trigger, UserError};
use prettytable::Table;
use regex::Regex;
use serde::Deserialize;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs;
use std::vec::Vec;

pub struct Configuration {
  pub actions: Vec<Action>,
  pub options: Options,
  pub last_command: Cell<Option<String>>,
}

/// Actions are executed when receiving a trigger.
#[derive(Deserialize)]
pub struct Action {
  trigger: Trigger,
  run: String,
  vars: Option<Vec<Var>>,
}

pub struct Options {
  pub before_run: BeforeRun,
  pub after_run: AfterRun,
}

pub struct BeforeRun {
  pub clear_screen: bool,
  pub newlines: u8,
}

#[derive(Deserialize)]
struct Var {
  name: String,
  source: VarSource,
  filter: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum VarSource {
  File,
  Line,
  CurrentOrAboveLineContent,
}

impl Configuration {
  pub fn get_command(&self, trigger: Trigger) -> Result<String> {
    if trigger.command == "repeatTest" {
      // HACK: taking the value out of the cell and putting a clone back into it
      //       because I don't understand how to implement Copy for the embedded String yet.
      let last_command = self.last_command.take();
      self.last_command.set(last_command.clone());
      match last_command {
        None => return Err(UserError::NoCommandToRepeat {}),
        Some(command) => return Ok(command),
      }
    }
    for action in &self.actions {
      if action.trigger.matches_client_trigger(&trigger)? {
        let command = self.format_run(action, &trigger)?;
        self.last_command.set(Some(command.clone()));
        return Ok(command);
      }
    }
    Err(UserError::UnknownTrigger {
      line: trigger.to_string(),
    })
  }

  /// replaces all placeholders in the given run string
  fn format_run(&self, action: &Action, trigger: &Trigger) -> Result<String> {
    let mut values: HashMap<&str, String> = HashMap::new();
    values.insert("command", trigger.command.clone());
    if trigger.file.is_some() {
      values.insert("file", trigger.file.as_ref().unwrap().clone());
    }
    if trigger.line.is_some() {
      values.insert("line", trigger.line.as_ref().unwrap().to_string());
    }
    if action.vars.is_some() {
      for var in action.vars.as_ref().unwrap() {
        values.insert(&var.name, calculate_var(var, &values)?);
      }
    }
    let mut replaced = action.run.clone();
    for (placeholder, replacement) in values {
      replaced = replace(&replaced, placeholder, &replacement);
    }
    Ok(replaced)
  }
}

#[allow(clippy::str_to_string, clippy::string_to_string)]
impl Display for Configuration {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut table = Table::new();
    table.add_row(prettytable::row!["TRIGGER", "RUN"]);
    for action in &self.actions {
      table.add_row(prettytable::row![action.trigger, action.run]);
    }
    table.printstd();
    f.write_str("Options:")?;
    f.write_fmt(format_args!(
      "- beforeRun.clearScreen: {}",
      self.options.before_run.clear_screen
    ))?;
    Ok(())
  }
}

impl Display for VarSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = match &self {
      VarSource::File => "file",
      VarSource::Line => "line",
      VarSource::CurrentOrAboveLineContent => "currentOrAboveLineContent",
    };
    write!(f, "{}", text)
  }
}

impl Options {
  pub fn defaults() -> Options {
    Options {
      before_run: BeforeRun {
        clear_screen: false,
        newlines: 0,
      },
      after_run: AfterRun {
        newlines: 0,
        indicator_lines: 3,
      },
    }
  }
}

pub struct AfterRun {
  pub newlines: u8,
  pub indicator_lines: u8,
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
      let mut line: u32 = values.get("line").unwrap().parse().unwrap();
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
  mod get_command {
    use super::super::super::{Action, Configuration};
    use super::super::*;
    use std::cell::Cell;

    #[test]
    fn test_all() {
      let config = Configuration {
        actions: vec![],
        options: Options {
          before_run: BeforeRun {
            clear_screen: false,
            newlines: 0,
          },
          after_run: AfterRun {
            newlines: 0,
            indicator_lines: 0,
          },
        },
        last_command: Cell::new(None),
      };
      let give = Trigger {
        command: "testAll".into(),
        file: None,
        line: None,
      };
      let have = config.get_command(give);
      assert!(have.is_err());
    }

    #[test]
    fn exact_match() {
      let action1 = Action {
        trigger: Trigger {
          command: "testFunction".into(),
          file: Some("filename1".into()),
          line: Some("*".into()),
        },
        run: String::from("action1 command"),
        vars: Some(vec![]),
      };
      let action2 = Action {
        trigger: Trigger {
          command: "testFunction".into(),
          file: Some("filename2".into()),
          line: Some("*".into()),
        },
        run: String::from("action2 command"),
        vars: Some(vec![]),
      };
      let action3 = Action {
        trigger: Trigger {
          command: "testFunction".into(),
          file: Some("filename3".into()),
          line: Some("*".into()),
        },
        run: String::from("action3 command"),
        vars: Some(vec![]),
      };
      let config = Configuration {
        actions: vec![action1, action2, action3],
        options: Options {
          before_run: BeforeRun {
            clear_screen: false,
            newlines: 0,
          },
          after_run: AfterRun {
            newlines: 0,
            indicator_lines: 0,
          },
        },
        last_command: Cell::new(None),
      };
      let give = Trigger {
        command: "testFunction".into(),
        file: Some("filename2".into()),
        line: Some("2".into()),
      };
      let have = config.get_command(give);
      assert_eq!(have, Ok(String::from("action2 command")));
    }

    #[test]
    fn no_match() {
      let action1 = Action {
        trigger: Trigger {
          command: "testFile".into(),
          file: Some("filename".into()),
          line: None,
        },
        run: String::from("action1 command"),
        vars: Some(vec![]),
      };
      let config = Configuration {
        actions: vec![action1],
        options: Options {
          before_run: BeforeRun {
            clear_screen: false,
            newlines: 0,
          },
          after_run: AfterRun {
            newlines: 0,
            indicator_lines: 0,
          },
        },
        last_command: Cell::new(None),
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("other filename".into()),
        line: None,
      };
      let have = config.get_command(give);
      assert!(have.is_err());
    }
  }

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
