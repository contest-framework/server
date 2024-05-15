use crate::config::AfterRun;
use serde::Deserialize;

#[derive(Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileAfterRun {
  pub newlines: Option<u8>,
  pub indicator_lines: Option<u8>,
}

impl FileAfterRun {
  pub fn into_domain(self) -> AfterRun {
    AfterRun {
      newlines: self.newlines.unwrap_or_default(),
      indicator_lines: self.newlines.unwrap_or_default(),
    }
  }
}

#[cfg(test)]
mod tests {

  mod to_domain {
    use super::super::FileAfterRun;
    use crate::config::AfterRun;

    #[test]
    fn empty() {
      let file_after_run = FileAfterRun {
        newlines: None,
        indicator_lines: None,
      };
      let have = file_after_run.into_domain();
      let want = AfterRun {
        newlines: 0,
        indicator_lines: 0,
      };
      assert_eq!(have, want);
    }

    #[test]
    fn has_content() {
      let file_after_run = FileAfterRun {
        newlines: Some(2),
        indicator_lines: Some(4),
      };
      let have = file_after_run.into_domain();
      let want = AfterRun {
        newlines: 2,
        indicator_lines: 4,
      };
      assert_eq!(have, want);
    }
  }
}
