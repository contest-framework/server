use crate::config::BeforeRun;
use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
  pub clear_screen: Option<bool>,
  pub newlines: Option<u8>,
}

impl FileBeforeRun {
  pub fn to_domain(self) -> BeforeRun {
    BeforeRun {
      clear_screen: self.clear_screen.unwrap_or_default(),
      newlines: self.newlines.unwrap_or_default(),
    }
  }
}
