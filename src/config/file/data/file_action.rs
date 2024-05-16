use super::FileVar;
use crate::config::{Action, Pattern, Var};
use crate::{Result, UserError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileAction {
  r#type: String,
  files: Option<String>,
  run: String,
  vars: Option<Vec<FileVar>>,
}

impl FileAction {
  /// converts the raw file data into the validated format
  pub fn into_domain(self) -> Result<Action> {
    let action_type = self.r#type.to_ascii_lowercase();
    let file_vars = self.vars.unwrap_or_default();
    let mut vars: Vec<Var> = Vec::with_capacity(file_vars.len());
    if self.run.is_empty() {
      return Err(UserError::RunCommandIsEmpty);
    }
    for file_var in file_vars {
      vars.push(file_var.into_domain()?);
    }
    if &action_type == "testall" {
      return Ok(Action {
        pattern: Pattern::TestAll,
        run: self.run,
        vars,
      });
    }
    let Some(files) = self.files else {
      return Err(UserError::MissingFilesInPattern);
    };
    if files.is_empty() {
      return Err(UserError::FilesIsEmpty);
    }
    let pattern = glob::Pattern::new(&files).map_err(|err| UserError::ConfigInvalidGlob {
      pattern: files,
      err: err.to_string(),
    })?;
    if &action_type == "testfile" {
      return Ok(Action {
        pattern: Pattern::TestFile { files: pattern },
        run: self.run,
        vars,
      });
    }
    if &action_type == "testfunction" {
      return Ok(Action {
        pattern: Pattern::TestFileLine { files: pattern },
        run: self.run,
        vars,
      });
    }
    Err(UserError::UnknownActionType { action_type })
  }
}

#[cfg(test)]
mod tests {

  mod into_domain {

    mod test_all {
      use super::super::super::FileAction;
      use crate::config::{Action, Pattern};
      use big_s::S;

      #[test]
      fn valid() {
        let file_action = FileAction {
          r#type: S("testAll"),
          files: None,
          run: S("make test"),
          vars: None,
        };
        let have = file_action.into_domain().unwrap();
        let want = Action {
          pattern: Pattern::TestAll,
          run: S("make test"),
          vars: vec![],
        };
        assert_eq!(have, want);
      }

      #[test]
      fn empty_run() {
        let file_action = FileAction {
          r#type: S("testAll"),
          files: None,
          run: S(""),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }
    }

    mod test_file {
      use super::super::super::FileAction;
      use crate::config::file::data::FileVar;
      use crate::config::{Action, Pattern, Var, VarSource};
      use big_s::S;

      #[test]
      fn valid_simple() {
        let file_action = FileAction {
          r#type: S("testFile"),
          files: Some(S("**/*.rs")),
          run: S("cargo test"),
          vars: None,
        };
        let have = file_action.into_domain().unwrap();
        let want = Action {
          pattern: Pattern::TestFile {
            files: glob::Pattern::new("**/*.rs").unwrap(),
          },
          run: S("cargo test"),
          vars: vec![],
        };
        assert_eq!(have, want);
      }

      #[test]
      fn valid_with_vars() {
        let file_action = FileAction {
          r#type: S("testFile"),
          files: Some(S("**/*.rs")),
          run: S("cargo test {{ my_var }}"),
          vars: Some(vec![FileVar {
            name: S("my_var"),
            source: VarSource::File,
            filter: S("^fn (.*) \\{"),
          }]),
        };
        let have = file_action.into_domain().unwrap();
        let want = Action {
          pattern: Pattern::TestFile {
            files: glob::Pattern::new("**/*.rs").unwrap(),
          },
          run: S("cargo test {{ my_var }}"),
          vars: vec![Var {
            name: S("my_var"),
            source: VarSource::File,
            filter: regex::Regex::new("^fn (.*) \\{").unwrap(),
          }],
        };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_files() {
        let file_action = FileAction {
          r#type: S("testFile"),
          files: None,
          run: S("make test"),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }

      #[test]
      fn empty_files() {
        let file_action = FileAction {
          r#type: S("testFile"),
          files: Some(S("")),
          run: S("make test"),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }

      #[test]
      fn empty_run() {
        let file_action = FileAction {
          r#type: S("testFile"),
          files: Some(S("**/*.rs")),
          run: S(""),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }
    }

    mod test_function {
      use super::super::super::FileAction;
      use crate::config::file::data::FileVar;
      use crate::config::{Action, Pattern, Var, VarSource};
      use big_s::S;

      #[test]
      fn valid_simple() {
        let file_action = FileAction {
          r#type: S("testFunction"),
          files: Some(S("**/*.rs")),
          run: S("cargo test"),
          vars: None,
        };
        let have = file_action.into_domain().unwrap();
        let want = Action {
          pattern: Pattern::TestFileLine {
            files: glob::Pattern::new("**/*.rs").unwrap(),
          },
          run: S("cargo test"),
          vars: vec![],
        };
        assert_eq!(have, want);
      }

      #[test]
      fn valid_with_vars() {
        let file_action = FileAction {
          r#type: S("testFunction"),
          files: Some(S("**/*.rs")),
          run: S("cargo test {{ my_var }}"),
          vars: Some(vec![FileVar {
            name: S("my_var"),
            source: VarSource::File,
            filter: S("^fn (.*) \\{"),
          }]),
        };
        let have = file_action.into_domain().unwrap();
        let want = Action {
          pattern: Pattern::TestFileLine {
            files: glob::Pattern::new("**/*.rs").unwrap(),
          },
          run: S("cargo test {{ my_var }}"),
          vars: vec![Var {
            name: S("my_var"),
            source: VarSource::File,
            filter: regex::Regex::new("^fn (.*) \\{").unwrap(),
          }],
        };
        assert_eq!(have, want);
      }

      #[test]
      fn missing_files() {
        let file_action = FileAction {
          r#type: S("testFunction"),
          files: None,
          run: S("make test"),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }

      #[test]
      fn empty_files() {
        let file_action = FileAction {
          r#type: S("testFunction"),
          files: Some(S("")),
          run: S("make test"),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }

      #[test]
      fn empty_run() {
        let file_action = FileAction {
          r#type: S("testFile"),
          files: Some(S("**/*.rs")),
          run: S(""),
          vars: None,
        };
        let have = file_action.into_domain();
        assert!(have.is_err());
      }
    }
  }
}
