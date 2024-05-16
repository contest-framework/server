//! A very simple templating engine that just replaces placeholders in the string template.

mod regex;
mod replace;
mod replace_all;

pub use regex::regex;
pub use replace::replace;
pub use replace_all::replace_all;
