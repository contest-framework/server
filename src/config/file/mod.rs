//! program configuration persisted in the config file

mod create;
pub mod data;
mod read;

pub use create::create;
use data::FileConfiguration;
#[cfg(test)]
pub use read::read;

/// filename of the Contest config file
const JSON_PATH: &str = ".testconfig.json";
const JSON5_PATH: &str = ".testconfig.json5";
