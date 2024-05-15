//! This module defines the data structures used in the config file.
//! This is for de-serializing the config file into Rust data structures.
//! We then convert the data structure to validated config data in Rust.

mod action;
mod file_after_run;
mod file_before_run;
mod file_configuration;
mod file_options;
mod file_var;

pub use action::FileAction;
pub use file_after_run::FileAfterRun;
pub use file_before_run::FileBeforeRun;
pub use file_configuration::FileConfiguration;
pub use file_options::FileOptions;
pub use file_var::FileVar;
