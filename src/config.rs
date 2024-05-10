//! program configuration loaded from the config file

use super::errors::UserError;
use super::trigger::Trigger;
use prettytable::Table;
use regex::Regex;
use serde::Deserialize;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io;
use std::vec::Vec;

/// Actions are executed when receiving a trigger.
#[derive(Deserialize)]
pub struct Action {
    trigger: Trigger,
    run: String,
    vars: Option<Vec<Var>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum VarSource {
    File,
    Line,
    CurrentOrAboveLineContent,
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

#[derive(Deserialize)]
struct Var {
    name: String,
    source: VarSource,
    filter: String,
}

pub struct Options {
    pub before_run: BeforeRun,
    pub after_run: AfterRun,
}

/// structure of options stored in the config file
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileOptions {
    before_run: Option<FileBeforeRun>,
    after_run: Option<FileAfterRun>,
}

impl Options {
    fn defaults() -> Options {
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

pub struct BeforeRun {
    pub clear_screen: bool,
    pub newlines: u8,
}

/// structure of the BeforeRun section in the configuration file
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileBeforeRun {
    clear_screen: Option<bool>,
    newlines: Option<u8>,
}

pub struct AfterRun {
    pub newlines: u8,
    pub indicator_lines: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileAfterRun {
    pub newlines: Option<u8>,
    pub indicator_lines: Option<u8>,
}

/// The structure of the configuration file.
#[derive(Deserialize)]
struct FileConfiguration {
    actions: Vec<Action>,
    options: Option<FileOptions>,
}

pub struct Configuration {
    pub actions: Vec<Action>,
    pub options: Options,
    pub last_command: Cell<Option<String>>,
}

pub fn from_file() -> Result<Configuration, UserError> {
    let file = match File::open(".testconfig.json") {
        Ok(config) => config,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => return Err(UserError::ConfigFileNotFound {}),
            _ => return Err(UserError::ConfigFileNotReadable { err: e.to_string() }),
        },
    };
    let file_config: FileConfiguration =
        serde_json::from_reader(file).map_err(|err| UserError::ConfigFileInvalidContent {
            err: err.to_string(),
        })?;
    Ok(Configuration::backfill_defaults(file_config))
}

pub fn create() -> Result<(), UserError> {
    fs::write(
        ".testconfig.json",
        r#"{
  "actions": [
    {
      "trigger": { "command": "testAll" },
      "run": "echo test all files"
    },

    {
      "trigger": {
        "command": "testFile",
        "file": "\\.rs$"
      },
      "run": "echo testing file {{file}}"
    },

    {
      "trigger": {
        "command": "testFunction",
        "file": "\\.ext$",
      },
      "run": "echo testing file {{file}} at line {{line}}"
    }
  ]
}"#,
    )
    .map_err(|e| UserError::CannotCreateConfigFile { err: e.to_string() })
}

impl Configuration {
    /// backfills missing values in the given FileConfiguration with default values
    fn backfill_defaults(file: FileConfiguration) -> Configuration {
        let defaults = Options::defaults();
        match file.options {
            None => Configuration {
                actions: file.actions,
                options: defaults,
                last_command: Cell::new(None),
            },
            Some(file_options) => Configuration {
                actions: file.actions,
                options: Options {
                    before_run: match file_options.before_run {
                        None => defaults.before_run,
                        Some(file_before_run) => BeforeRun {
                            clear_screen: file_before_run
                                .clear_screen
                                .unwrap_or(defaults.before_run.clear_screen),
                            newlines: file_before_run
                                .newlines
                                .unwrap_or(defaults.before_run.newlines),
                        },
                    },
                    after_run: match file_options.after_run {
                        None => defaults.after_run,
                        Some(file_after_run) => AfterRun {
                            indicator_lines: file_after_run
                                .indicator_lines
                                .unwrap_or(defaults.after_run.indicator_lines),
                            newlines: file_after_run
                                .newlines
                                .unwrap_or(defaults.after_run.newlines),
                        },
                    },
                },
                last_command: Cell::new(None),
            },
        }
    }

    pub fn get_command(&self, trigger: Trigger) -> Result<String, UserError> {
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
    fn format_run(&self, action: &Action, trigger: &Trigger) -> Result<String, UserError> {
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

fn calculate_var(var: &Var, values: &HashMap<&str, String>) -> Result<String, UserError> {
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

fn filter(text: &str, filter: &str) -> Result<String, UserError> {
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

//
// ----------------------------------------------------------------------------
//

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod backfill_defaults {
        use super::super::*;

        #[test]
        fn no_options() {
            let file_config = FileConfiguration {
                actions: vec![],
                options: None,
            };
            let config = Configuration::backfill_defaults(file_config);
            assert!(!config.options.before_run.clear_screen);
            assert_eq!(config.options.before_run.newlines, 0);
            assert_eq!(config.options.after_run.indicator_lines, 3);
            assert_eq!(config.options.after_run.newlines, 0);
        }

        #[test]
        fn some_options() {
            let file_config = FileConfiguration {
                actions: vec![],
                options: Some(FileOptions {
                    before_run: Some(FileBeforeRun {
                        clear_screen: Some(true),
                        newlines: None,
                    }),
                    after_run: None,
                }),
            };
            let config = Configuration::backfill_defaults(file_config);
            assert!(config.options.before_run.clear_screen);
            assert_eq!(config.options.before_run.newlines, 0);
            assert_eq!(config.options.after_run.indicator_lines, 3);
            assert_eq!(config.options.after_run.newlines, 0);
        }
    }

    #[cfg(test)]
    mod get_command {
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
        use super::super::*;

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
