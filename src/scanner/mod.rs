//! finds regular expression matches in the content of files on disk

mod scan_string;

pub use scan_string::{scan_file_upwards, scan_string_upwards};
