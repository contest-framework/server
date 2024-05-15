use crate::config::BeforeRun;
use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBeforeRun {
  pub clear_screen: Option<bool>,
  pub newlines: Option<u8>,
}

impl FileBeforeRun {
  pub fn to_domain(self) -> BeforeRun {
    BeforeRun {
      clear_screen: self.clear_screen.unwrap_or_default(),
      newlines: self.newlines.unwrap_or_default(),
    }
  }
}

#[cfg(test)]
mod tests {

  mod to_domain {
    use super::super::FileBeforeRun;
    use crate::config::BeforeRun;

    #[test]
    fn empty() {
      let file_after_run = FileBeforeRun {
        newlines: None,
        clear_screen: None,
      };
      let have = file_after_run.to_domain();
      let want = BeforeRun {
        newlines: 0,
        clear_screen: false,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn has_content() {
      let file_after_run = FileBeforeRun {
        newlines: Some(2),
        clear_screen: Some(true),
      };
      let have = file_after_run.to_domain();
      let want = BeforeRun {
        newlines: 2,
        clear_screen: true,
      };
      assert_eq!(have, want);
    }
  }
}
