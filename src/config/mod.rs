//! everything around configuring Tertestrial

mod data;
pub mod file;
mod pattern;

pub use data::{Action, AfterRun, BeforeRun, Configuration, Options, Trigger, Var};
pub use pattern::Pattern;
