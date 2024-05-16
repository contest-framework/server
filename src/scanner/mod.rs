//! finds regular expression matches in the content of files on disk

mod scan_file;
mod scan_string;

pub use scan_file::scan_file_upwards;
pub use scan_string::scan_string_upwards;
