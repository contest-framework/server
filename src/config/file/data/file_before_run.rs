use serde::Deserialize;

/// low-level, unvalidated `BeforeRun` data exactly how it is stored in the config file
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
  pub clear_screen: Option<bool>,
  pub newlines: Option<u8>,
}
