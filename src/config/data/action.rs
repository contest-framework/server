use super::{Pattern, Var};

/// Actions are executed when receiving a command.
#[derive(Debug, PartialEq)]
pub struct Action {
  pub pattern: Pattern,
  pub run: String,
  pub vars: Vec<Var>,
}
