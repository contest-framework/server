//! program configuration persisted in the config file

mod create;
mod data;
mod read;

pub use create::create;
use data::JsonContent;
pub use read::read;

/// filename of the Tertestrial config file
const PATH: &str = ".testconfig.json";
