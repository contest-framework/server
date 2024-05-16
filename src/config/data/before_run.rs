#[derive(Debug, Default, Eq, PartialEq)]
pub struct BeforeRun {
  pub clear_screen: bool,
  pub newlines: u8,
}
