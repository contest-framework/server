use crate::config::VarSource;
use serde::Deserialize;

/// low-level, unvalidated `Action` data exactly how it is stored in the config file
#[derive(Deserialize)]
pub struct FileAction {
  pub r#type: String,
  pub files: Option<String>,
  pub run: String,
  pub vars: Option<Vec<FileVar>>,
}

/// low-level, unvalidated `AfterRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileAfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
  pub print_result: Option<bool>,
}

/// low-level, unvalidated `BeforeRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
  pub clear_screen: Option<bool>,
  pub newlines: Option<u8>,
}

/// low-level, unvalidated `Configuration` data exactly how it is stored in the config file
#[derive(Deserialize)]
pub struct FileConfiguration {
  pub actions: Vec<FileAction>,
  pub options: Option<FileOptions>,
}

/// low-level, unvalidated `Options` data exactly how it is stored in the config file
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileOptions {
  pub before_run: Option<FileBeforeRun>,
  pub after_run: Option<FileAfterRun>,
}

/// low-level, unvalidated `Var` data exactly how it is stored in the config file
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct FileVar {
  pub name: String,
  pub source: VarSource,
  pub filter: String,
}
