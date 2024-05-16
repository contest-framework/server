//! A very simple templating engine that just replaces placeholders in the string template.

mod replace;
mod replace_all;

pub use replace::replace;
pub use replace_all::replace_all;
