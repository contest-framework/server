//! data structures as they are in the file

use crate::config::VarSource;
use schemars::JsonSchema;
use serde::Deserialize;

/// configuration data for <https://github.com/contest-framework/server>
#[derive(Deserialize, JsonSchema)]
pub struct FileConfiguration {
  /// define the tests that Contest will run for you
  pub actions: Vec<FileAction>,
  /// configure the layout and behavior
  pub options: Option<FileOptions>,
}

/// a particular test
#[derive(Deserialize, JsonSchema)]
pub struct FileAction {
  pub r#type: String,
  /// the files for which this command applies as a glob expression
  pub files: Option<String>,
  /// the command to run
  pub run: String,
  /// define additional variables to use in the "run" string
  pub vars: Option<Vec<FileVar>>,
}

/// an additional variable that gets derived from the file content
#[derive(Deserialize, Debug, Eq, JsonSchema, PartialEq)]
pub struct FileVar {
  /// name of the variable, available as "{{ name }}" later
  pub name: String,
  /// the location in the file
  pub source: VarSource,
  /// how the variable gets computed
  pub filter: String,
}

/// low-level, unvalidated `Options` data exactly how it is stored in the config file
#[derive(Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileOptions {
  pub before_run: Option<FileBeforeRun>,
  pub after_run: Option<FileAfterRun>,
}

/// low-level, unvalidated `BeforeRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
  pub clear_screen: Option<bool>,
  pub newlines: Option<u8>,
}

/// low-level, unvalidated `AfterRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileAfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
  pub print_result: Option<bool>,
}
