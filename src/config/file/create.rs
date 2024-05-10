use crate::{Result, UserError};
use std::fs;

pub fn create() -> Result<()> {
    fs::write(
        ".testconfig.json",
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
