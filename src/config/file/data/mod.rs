//! This module defines the data structures used in the config file.
//! This is for de-serializing the config file into Rust data structures.
//! We then convert the data structure to validated config data in Rust.

mod action;
mod after_run;
mod before_run;
mod configuration;
mod options;
mod var;

pub use action::FileAction;
pub use after_run::FileAfterRun;
pub use before_run::FileBeforeRun;
pub use configuration::FileConfiguration;
pub use options::FileOptions;
pub use var::FileVar;
