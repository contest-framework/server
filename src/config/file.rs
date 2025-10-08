//! data structures as they are in the file

use crate::config::VarSource;
use schemars::JsonSchema;
use schemars::schema::{InstanceType, NumberValidation, SchemaObject};
use serde::Deserialize;
use std::fmt::Display;

/// Schema helper for Option<usize> that generates standard JSON Schema integer without "format": "uint"
fn option_usize_schema(_gen: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
  SchemaObject {
    instance_type: Some(vec![InstanceType::Integer, InstanceType::Null].into()),
    number: Some(Box::new(NumberValidation {
      minimum: Some(0.0),
      ..Default::default()
    })),
    ..Default::default()
  }
  .into()
}

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
  pub r#type: ActionType,
  /// the files for which this command applies as a glob expression
  pub files: Option<String>,
  /// the command to run
  pub run: String,
  /// define additional variables to use in the "run" string
  pub vars: Option<Vec<FileVar>>,
  /// human-readable description of this action
  pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ActionType {
  TestAll,
  TestFile,
  TestFileLine,
}

impl Display for ActionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      ActionType::TestAll => "test-all",
      ActionType::TestFile => "test-file",
      ActionType::TestFileLine => "test-file-line",
    })
  }
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

#[derive(Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileOptions {
  /// configure behavior before Contest runs a test
  pub before_run: Option<FileBeforeRun>,
  /// configure behavior after Contest runs a test
  pub after_run: Option<FileAfterRun>,
}

#[derive(Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
  /// whether to clear the screen before a test run
  pub clear_screen: Option<bool>,
  /// how many newlines to print before a test run
  #[schemars(schema_with = "option_usize_schema", default)]
  pub newlines: Option<usize>,
}

#[derive(Default, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileAfterRun {
  /// how many newlines to print after a test run
  #[schemars(schema_with = "option_usize_schema", default)]
  pub newlines: Option<usize>,
  /// how many indicator lines (red or green) to print after a test run
  #[schemars(schema_with = "option_usize_schema", default)]
  pub indicator_lines: Option<usize>,
  /// whether to print "SUCCESS" or "FAILED" after a test run
  pub print_result: Option<bool>,
}
