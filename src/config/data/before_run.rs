use crate::config::file::data::FileBeforeRun;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct BeforeRun {
  pub clear_screen: bool,
  pub newlines: u8,
}

impl From<FileBeforeRun> for BeforeRun {
  fn from(value: FileBeforeRun) -> Self {
    BeforeRun {
      clear_screen: value.clear_screen.unwrap_or_default(),
      newlines: value.newlines.unwrap_or_default(),
    }
  }
}

#[cfg(test)]
mod tests {

  mod into_domain {
    use super::super::FileBeforeRun;
    use crate::config::BeforeRun;

    #[test]
    fn empty() {
      let file_before_run = FileBeforeRun {
        newlines: None,
        clear_screen: None,
      };
      let have = BeforeRun::from(file_before_run);
      let want = BeforeRun { newlines: 0, clear_screen: false };
      assert_eq!(have, want);
    }

    #[test]
    fn has_content() {
      let file_before_run = FileBeforeRun {
        newlines: Some(2),
        clear_screen: Some(true),
      };
      let have = BeforeRun::from(file_before_run);
      let want = BeforeRun { newlines: 2, clear_screen: true };
      assert_eq!(have, want);
    }
  }
}
