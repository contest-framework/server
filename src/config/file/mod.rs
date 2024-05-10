//! program configuration persisted in the config file

mod create;
mod read;

use super::Action;
pub use create::create;
pub use read::read;
use serde::Deserialize;

/// filename of the Tertestrial config file
const PATH: &str = ".testconfig.json";

/// The structure of the configuration file.
#[derive(Deserialize)]
pub struct Content {
  actions: Vec<Action>,
  options: Option<FileOptions>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileOptions {
  before_run: Option<BeforeRun>,
  after_run: Option<AfterRun>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeforeRun {
  clear_screen: Option<bool>,
  newlines: Option<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
}
