use crate::{Result, UserError};
use std::fs;

use super::PATH;

// creates an example config file on disk
pub fn create() -> Result<()> {
    fs::write(
        PATH,
        r#"{
  "actions": [
    {
      "trigger": { "command": "testAll" },
      "run": "echo test all files"
    },

    {
      "trigger": {
        "command": "testFile",
        "file": "\\.rs$"
      },
      "run": "echo testing file {{file}}"
    },

    {
      "trigger": {
        "command": "testFunction",
        "file": "\\.ext$",
      },
      "run": "echo testing file {{file}} at line {{line}}"
    }
  ]
}"#,
    )
    .map_err(|e| UserError::CannotCreateConfigFile { err: e.to_string() })
}
