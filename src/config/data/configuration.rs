use super::{Action, Options};
use crate::client::Trigger;
use crate::Result;
use crate::UserError;
use prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;
use prettytable::Table;
use std::collections::HashMap;
use std::fmt::{self, Display};

pub struct Configuration {
  pub actions: Vec<Action>,
  pub options: Options,
}

impl Configuration {
  pub fn get_command(&self, trigger: Trigger, last_command: &mut Option<String>) -> Result<String> {
    if trigger == Trigger::RepeatLastTest {
      match last_command {
        Some(command) => return Ok(command.to_owned()),
        None => return Err(UserError::NoCommandToRepeat {}),
      }
    }
    for action in &self.actions {
      if action.trigger.matches_client_trigger(&trigger)? {
        let command = self.format_run(action, &trigger)?;
        last_command.replace(command.clone());
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
    table.set_format(*FORMAT_NO_BORDER_LINE_SEPARATOR);
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

#[cfg(test)]
mod tests {

  #[cfg(test)]
  mod get_command {
    use super::super::super::{Action, BeforeRun, Configuration};
    use super::super::*;

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
      };
      let give = Trigger {
        command: "testAll".into(),
        file: None,
        line: None,
      };
      let mut last_command: Option<String> = None;
      let have = config.get_command(give, &mut last_command);
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
      };
      let give = Trigger {
        command: "testFunction".into(),
        file: Some("filename2".into()),
        line: Some("2".into()),
      };
      let mut last_command: Option<String> = None;
      let have = config.get_command(give, &mut last_command);
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
      };
      let give = Trigger {
        command: "testFile".into(),
        file: Some("other filename".into()),
        line: None,
      };
      let mut last_command: Option<String> = None;
      let have = config.get_command(give, &mut last_command);
      assert!(have.is_err());
    }
  }
}
