use serde::Deserialize;

/// low-level, unvalidated `AfterRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileAfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
  pub print_result: Option<bool>,
}
