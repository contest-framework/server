use crate::UserError;

pub fn print_error(err: &UserError) {
  let (msg, guidance) = err.messages();
  println!("Error: {msg}");
  if let Some(guidance) = guidance {
    println!("{guidance}");
  }
}

#[must_use]
pub fn error_color(success: bool) -> termcolor::Color {
  if success { termcolor::Color::Green } else { termcolor::Color::Red }
}
