use super::{Pattern, Var};
use crate::UserError;
use crate::config::file::FileAction;

/// Actions are executed when receiving a command.
#[derive(Debug, PartialEq)]
pub struct Action {
  pub pattern: Pattern,
  pub run: String,
  pub vars: Vec<Var>,
}

impl TryFrom<FileAction> for Action {
  type Error = UserError;

  fn try_from(value: FileAction) -> Result<Self, Self::Error> {
    let action_type = value.r#type.to_ascii_lowercase();
    let file_vars = value.vars.unwrap_or_default();
    let mut vars: Vec<Var> = Vec::with_capacity(file_vars.len());
    if value.run.is_empty() {
      return Err(UserError::RunCommandIsEmpty);
    }
    for file_var in file_vars {
      vars.push(Var::try_from(file_var)?);
    }
    if &action_type == "testall" {
      return Ok(Action {
        pattern: Pattern::TestAll,
        run: value.run,
        vars,
      });
    }
    let Some(files) = value.files else {
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
        run: value.run,
        vars,
      });
    }
    if &action_type == "testfileline" {
      return Ok(Action {
        pattern: Pattern::TestFileLine { files: pattern },
        run: value.run,
        vars,
      });
    }
    Err(UserError::UnknownActionType { action_type })
  }
}

#[cfg(test)]
mod tests {

  mod try_from {

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
        let have = Action::try_from(file_action).unwrap();
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
        let have = Action::try_from(file_action);
        assert!(have.is_err());
      }
    }

    mod test_file {
      use super::super::super::FileAction;
      use crate::config::file::FileVar;
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
        let have = Action::try_from(file_action).unwrap();
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
        let have = Action::try_from(file_action).unwrap();
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
        let have = Action::try_from(file_action);
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
        let have = Action::try_from(file_action);
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
        let have = Action::try_from(file_action);
        assert!(have.is_err());
      }
    }

    mod test_function {
      use super::super::super::FileAction;
      use crate::config::file::FileVar;
      use crate::config::{Action, Pattern, Var, VarSource};
      use big_s::S;

      #[test]
      fn valid_simple() {
        let file_action = FileAction {
          r#type: S("testFileLine"),
          files: Some(S("**/*.rs")),
          run: S("cargo test"),
          vars: None,
        };
        let have = Action::try_from(file_action).unwrap();
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
          r#type: S("testFileLine"),
          files: Some(S("**/*.rs")),
          run: S("cargo test {{ my_var }}"),
          vars: Some(vec![FileVar {
            name: S("my_var"),
            source: VarSource::File,
            filter: S("^fn (.*) \\{"),
          }]),
        };
        let have = Action::try_from(file_action).unwrap();
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
          r#type: S("testFileLine"),
          files: None,
          run: S("make test"),
          vars: None,
        };
        let have = Action::try_from(file_action);
        assert!(have.is_err());
      }

      #[test]
      fn empty_files() {
        let file_action = FileAction {
          r#type: S("testFileLine"),
          files: Some(S("")),
          run: S("make test"),
          vars: None,
        };
        let have = Action::try_from(file_action);
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
        let have = Action::try_from(file_action);
        assert!(have.is_err());
      }
    }
  }
}
