use super::{Trigger, Var};

/// Actions are executed when receiving a trigger.
#[derive(Debug, PartialEq)]
pub struct Action {
  pub trigger: Trigger,
  pub run: String,
  pub vars: Vec<Var>,
}
