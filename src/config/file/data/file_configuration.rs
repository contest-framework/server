use super::{FileAction, FileOptions};
use crate::Result;
use crate::config::{Action, Configuration};
use serde::Deserialize;

/// low-level, unvalidated `Configuration` data exactly how it is stored in the config file
#[derive(Deserialize)]
pub struct FileConfiguration {
  actions: Vec<FileAction>,
  options: Option<FileOptions>,
}

impl FileConfiguration {
  pub fn into_domain(self) -> Result<Configuration> {
    let mut actions: Vec<Action> = Vec::with_capacity(self.actions.len());
    for json_action in self.actions {
      actions.push(json_action.into_domain()?);
    }
    Ok(Configuration {
      actions,
      options: self.options.unwrap_or_default().into_domain(),
    })
  }
}

#[cfg(test)]
mod tests {

  mod into_domain {
    use crate::config::file::data::{FileAction, FileConfiguration};
    use crate::config::{Action, Configuration, Options, Pattern};
    use big_s::S;

    #[test]
    fn simple() {
      let file_config = FileConfiguration {
        actions: vec![FileAction {
          r#type: S("testFile"),
          files: Some(S("*.rs")),
          run: S("make test"),
          vars: None,
        }],
        options: None,
      };
      let have = file_config.into_domain().unwrap();
      let want = Configuration {
        actions: vec![Action {
          pattern: Pattern::TestFile {
            files: glob::Pattern::new("*.rs").unwrap(),
          },
          run: S("make test"),
          vars: vec![],
        }],
        options: Options::default(),
      };
      assert_eq!(have, want);
    }
  }
}
