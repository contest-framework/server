use crate::UserError;

pub fn print_error(err: &UserError) {
  let (msg, desc) = err.messages();
  println!("Error: {msg}");
  println!("{desc}");
}

#[must_use]
pub fn error_color(success: bool) -> termcolor::Color {
  if success { termcolor::Color::Green } else { termcolor::Color::Red }
}
