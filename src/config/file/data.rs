use crate::config::data::Options;
use crate::config::Configuration;
use serde::Deserialize;

/// The structure of the configuration file.
#[derive(Deserialize)]
pub struct JsonContent {
  actions: Vec<JsonAction>,
  options: Option<FileOptions>,
}

#[derive(Deserialize)]
struct JsonAction {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileOptions {
  before_run: Option<JsonBeforeRun>,
  after_run: Option<JsonAfterRun>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonBeforeRun {
  clear_screen: Option<bool>,
  newlines: Option<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonAfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
}

impl JsonContent {
  /// backfills missing values in the given FileConfiguration with default values
  pub fn to_configuration(&self) -> Configuration {
    let defaults = Options::defaults();
    match self.options {
      None => Configuration {
        actions: self.actions,
        options: defaults,
      },
      Some(file_options) => Configuration {
        actions: self.actions,
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
      },
    }
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
