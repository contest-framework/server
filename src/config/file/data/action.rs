use crate::config::{Action, Var};
use crate::{Result, UserError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileAction {
  r#type: String,
  files: Option<String>,
  run: String,
  vars: Option<Vec<Var>>,
  desc: Option<String>,
}

impl FileAction {
  fn to_domain(self) -> Result<Action> {
    match self.r#type.to_ascii_lowercase().as_str() {
      "testall" => Ok(Action::TestAll { run: self.run }),
      "testfile" => {
        let Some(files) = self.files else {
          return Err(UserError::MissingFilesInTestFile);
        };
        let pattern =
          glob::Pattern::new(&files).map_err(|err| UserError::ConfigInvalidGlobPattern {
            pattern: files,
            err: err.to_string(),
          })?;
        let vars = self.vars.unwrap_or_default();
        Ok(Action::TestFile {
          files: pattern,
          vars,
          run: self.run,
        })
      }
    }
  }
}
