use super::super::Configuration;
use super::{Content, PATH};
use crate::config::data::{AfterRun, BeforeRun, Options};
use crate::{Result, UserError};
use std::cell::Cell;
use std::fs::File;
use std::io;

pub fn read() -> Result<Configuration> {
    let file = match File::open(PATH) {
        Ok(config) => config,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => return Err(UserError::ConfigFileNotFound {}),
            _ => return Err(UserError::ConfigFileNotReadable { err: e.to_string() }),
        },
    };
    let file_config: Content =
        serde_json::from_reader(file).map_err(|err| UserError::ConfigFileInvalidContent {
            err: err.to_string(),
        })?;
    Ok(backfill_defaults(file_config))
}

/// backfills missing values in the given FileConfiguration with default values
fn backfill_defaults(file: Content) -> Configuration {
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

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod backfill_defaults {
        use super::super::super::FileOptions;
        use super::super::backfill_defaults;
        use crate::config::file::{self, BeforeRun, Content};

        #[test]
        fn no_options() {
            let file_config = file::Content {
                actions: vec![],
                options: None,
            };
            let config = backfill_defaults(file_config);
            assert!(!config.options.before_run.clear_screen);
            assert_eq!(config.options.before_run.newlines, 0);
            assert_eq!(config.options.after_run.indicator_lines, 3);
            assert_eq!(config.options.after_run.newlines, 0);
        }

        #[test]
        fn some_options() {
            let file_config = Content {
                actions: vec![],
                options: Some(FileOptions {
                    before_run: Some(BeforeRun {
                        clear_screen: Some(true),
                        newlines: None,
                    }),
                    after_run: None,
                }),
            };
            let config = backfill_defaults(file_config);
            assert!(config.options.before_run.clear_screen);
            assert_eq!(config.options.before_run.newlines, 0);
            assert_eq!(config.options.after_run.indicator_lines, 3);
            assert_eq!(config.options.after_run.newlines, 0);
        }
    }
}
