#[derive(Debug, Eq, PartialEq)]
pub struct AfterRun {
  pub newlines: u8,
  pub indicator_lines: u8,
  pub print_result: bool,
}

impl Default for AfterRun {
  fn default() -> Self {
    Self {
      newlines: 0,
      indicator_lines: 0,
      print_result: true,
    }
  }
}
