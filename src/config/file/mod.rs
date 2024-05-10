//! program configuration persisted in the config file

mod create;
mod read;

use super::Action;
pub use create::create;
pub use read::read;
use serde::Deserialize;

/// The structure of the configuration file.
#[derive(Deserialize)]
pub struct Content {
    actions: Vec<Action>,
    options: Option<FileOptions>,
}

/// structure of options stored in the config file
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileOptions {
    before_run: Option<FileBeforeRun>,
    after_run: Option<FileAfterRun>,
}

/// structure of the BeforeRun section in the configuration file
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
    clear_screen: Option<bool>,
    newlines: Option<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileAfterRun {
    pub newlines: Option<u8>,
    pub indicator_lines: Option<u8>,
}
