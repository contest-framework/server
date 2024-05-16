//! finds regular expression matches in the content of files on disk

mod file_upwards;
mod string_upwards;

pub use file_upwards::file_upwards;
pub use string_upwards::string_upwards;
