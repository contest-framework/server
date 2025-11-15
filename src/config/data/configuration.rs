use super::{Action, Options};
use crate::client::Trigger;
use crate::config::file::FileConfiguration;
use crate::{Result, UserError, template};
use ahash::AHashMap;
use prettytable::Table;
use prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;
use std::fmt::{self, Display};
use std::{fs, io};

/// filename of the Contest config file
pub const JSON_PATH: &str = "contest.json";

#[derive(Debug, Default, PartialEq)]
pub struct Configuration {
  pub actions: Vec<Action>,
  pub options: Options,
}

impl Configuration {
  // creates an example config file on disk
  pub fn create() -> Result<()> {
    let example_content = r#"{
  "$schema": "https://raw.githubusercontent.com/contest-framework/server/refs/heads/main/documentation/schema.json",
  "actions": [
    {
      "type": "test-all",
      "run": "echo test all files"
    },
    {
      "type": "test-file",
      "files": "**/*.ext",
      "run": "echo testing file {{file}}"
    },
    {
      "type": "test-file-line",
      "files": "**/*.ext",
      "run": "echo testing file {{file}} at line {{line}}"
    }
  ],
  "options": {
    "beforeRun": {
      "clearScreen": true,
      "newlines": 0
    },
    "afterRun": {
      "newlines": 1,
      "indicatorLines": 3,
      "indicatorBackground": true,
      "printResult": false
    }
  }
}"#;
    fs::write(JSON_PATH, example_content).map_err(|e| UserError::CannotCreateConfigFile { err: e.to_string() })?;
    println!("Created config file \"{JSON_PATH}\"");
    Ok(())
  }

  pub fn get_command(&self, trigger: &Trigger, last_command: &mut Option<String>) -> Result<String> {
    if trigger == &Trigger::RepeatLastTest {
      match last_command {
        Some(command) => return Ok(command.to_owned()),
        None => return Err(UserError::NoCommandToRepeat {}),
      }
    }
    if let Trigger::CustomCommand { run: command } = trigger {
      return Ok(command.to_owned());
    }
    for action in &self.actions {
      if action.pattern.matches_trigger(trigger) {
        return format_run(action, trigger);
      }
    }
    Err(UserError::UnknownTrigger { source: trigger.to_string() })
  }

  pub fn read() -> Result<Configuration> {
    let file_content = match fs::read_to_string(JSON_PATH) {
      Ok(file) => file,
      Err(err) if err.kind() == io::ErrorKind::NotFound => match fs::read_to_string(JSON_PATH) {
        Ok(file) => file,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Configuration::default()),
        Err(err) => return Err(UserError::ConfigFileError { err: err.to_string() }),
      },
      Err(err) => return Err(UserError::ConfigFileError { err: err.to_string() }),
    };
    let file_data: FileConfiguration = json5::from_str(&file_content).map_err(|err| UserError::ConfigFileInvalidContent { err: err.to_string() })?;
    Configuration::try_from(file_data)
  }
}

#[allow(clippy::str_to_string, clippy::string_to_string)]
impl Display for Configuration {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut table = Table::new();
    table.set_format(*FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.add_row(prettytable::row!["TRIGGER", "RUN"]);
    for action in &self.actions {
      table.add_row(prettytable::row![action.pattern, action.run]);
    }
    table.printstd();
    f.write_str("Options:\n")?;
    f.write_fmt(format_args!("- beforeRun.clearScreen: {}\n", self.options.before_run.clear_screen))?;
    Ok(())
  }
}

impl TryFrom<FileConfiguration> for Configuration {
  type Error = UserError;

  fn try_from(value: FileConfiguration) -> std::result::Result<Self, Self::Error> {
    let mut actions: Vec<Action> = Vec::with_capacity(value.actions.len());
    for json_action in value.actions {
      let action = Action::try_from(json_action)?;
      actions.push(action);
    }
    Ok(Configuration {
      actions,
      options: Options::from(value.options.unwrap_or_default()),
    })
  }
}

/// replaces all placeholders in the given run string
fn format_run(action: &Action, trigger: &Trigger) -> Result<String> {
  let mut values: AHashMap<&str, String> = AHashMap::new();
  if let Trigger::TestFile { file } = &trigger {
    values.insert("file", file.to_owned());
  }
  if let Trigger::TestFileLine { file, line } = &trigger {
    values.insert("file", file.to_owned());
    values.insert("line", line.to_string());
  }
  for var in &action.vars {
    values.insert(&var.name, var.calculate_var(&values)?);
  }
  template::replace_all(&action.run, &values)
}

#[cfg(test)]
mod tests {

  mod try_from {
    use crate::config::file::{ActionType, FileAction, FileConfiguration};
    use crate::config::{Action, Configuration, Options, Pattern};
    use big_s::S;

    #[test]
    fn simple() {
      let file_config = FileConfiguration {
        actions: vec![FileAction {
          r#type: ActionType::TestFile,
          files: Some(S("*.rs")),
          run: S("make test"),
          vars: None,
          comment: None,
        }],
        options: None,
      };
      let have = Configuration::try_from(file_config).unwrap();
      let want = Configuration {
        actions: vec![Action {
          pattern: Pattern::TestFile {
            files: glob::Pattern::new("*.rs").unwrap(),
          },
          run: S("make test"),
          vars: vec![],
        }],
        options: Options::default(),
      };
      assert_eq!(have, want);
    }
  }

  #[cfg(test)]
  mod get_command {
    use super::super::super::{Action, Configuration};
    use super::super::*;
    use crate::config::Pattern;
    use big_s::S;

    #[test]
    fn exact_match() {
      let action1 = Action {
        pattern: Pattern::TestFileLine {
          files: glob::Pattern::new("filename1").unwrap(),
        },
        run: String::from("action1 command"),
        vars: vec![],
      };
      let action2 = Action {
        pattern: Pattern::TestFileLine {
          files: glob::Pattern::new("filename2").unwrap(),
        },
        run: String::from("action2 command"),
        vars: vec![],
      };
      let action3 = Action {
        pattern: Pattern::TestFileLine {
          files: glob::Pattern::new("filename3").unwrap(),
        },
        run: String::from("action3 command"),
        vars: vec![],
      };
      let config = Configuration {
        actions: vec![action1, action2, action3],
        ..Configuration::default()
      };
      let trigger = Trigger::TestFileLine { file: S("filename2"), line: 2 };
      let mut last_command: Option<String> = None;
      let have = config.get_command(&trigger, &mut last_command);
      assert_eq!(have, Ok(String::from("action2 command")));
    }

    #[test]
    fn no_match() {
      let action1 = Action {
        pattern: Pattern::TestFile {
          files: glob::Pattern::new("*.rs").unwrap(),
        },
        run: String::from("action1 command"),
        vars: vec![],
      };
      let config = Configuration {
        actions: vec![action1],
        ..Configuration::default()
      };
      let give = Trigger::TestFile { file: S("other_filename") };
      let mut last_command: Option<String> = None;
      let have = config.get_command(&give, &mut last_command);
      assert!(have.is_err());
    }

    #[test]
    fn no_actions() {
      let config = Configuration {
        actions: vec![],
        ..Configuration::default()
      };
      let trigger = Trigger::TestAll;
      let mut last_command: Option<String> = None;
      let have = config.get_command(&trigger, &mut last_command);
      assert!(have.is_err());
    }
  }
}
