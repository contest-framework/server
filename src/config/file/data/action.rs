use crate::config::{Action, Trigger, Var};
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
  pub fn to_domain(self) -> Result<Action> {
    let action_type = self.r#type.to_ascii_lowercase();
    let vars = self.vars.unwrap_or_default();
    if &action_type == "testall" {
      return Ok(Action {
        trigger: Trigger::TestAll,
        run: self.run,
        vars,
      });
    }
    let Some(files) = self.files else {
      return Err(UserError::MissingFilesInTestFile);
    };
    let pattern =
      glob::Pattern::new(&files).map_err(|err| UserError::ConfigInvalidGlobPattern {
        pattern: files,
        err: err.to_string(),
      })?;
    if &action_type == "testfile" {
      return Ok(Action {
        trigger: Trigger::TestFile { files: pattern },
        run: self.run,
        vars,
      });
    }
    if &action_type == "testFunction" {
      return Ok(Action {
        trigger: Trigger::TestFileLine { files: pattern },
        run: self.run,
        vars,
      });
    }
    Err(UserError::UnknownActionType {
      action_type: action_type,
    })
  }
}
