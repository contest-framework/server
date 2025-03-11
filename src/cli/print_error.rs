use crate::{UserError, subshell};

pub fn print_error(err: &UserError) {
  let (msg, guidance) = err.messages();
  println!("Error: {msg}");
  if let Some(guidance) = guidance {
    println!("{guidance}");
  }
}

#[must_use]
pub fn error_color(success: &subshell::Outcome) -> termcolor::Color {
  match success {
    subshell::Outcome::TestPass => termcolor::Color::Green,
    subshell::Outcome::TestFail => termcolor::Color::Red,
  }
}
