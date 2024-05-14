use regex::Regex;

/// a pattern in the config file that matches incoming commands from the client
#[derive(Deserialize)]
pub struct Pattern {
  pub command: Regex,
  pub file: Option<Regex>,
}
