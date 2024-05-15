//! everything around configuring Tertestrial

mod data;
pub mod file;
mod pattern;

pub use data::{Action, AfterRun, BeforeRun, Configuration, Options, Pattern, Var, VarSource};
pub use pattern::Pattern;
