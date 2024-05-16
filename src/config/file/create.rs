use super::PATH;
use crate::{Result, UserError};
use std::fs;

const EXAMPLE_CONTENT: &str = r#"{
  "actions": [
    {
      "type": "testAll",
      "run": "echo test all files"
    },
    {
      "type": "testFile",
      "file": "\\.rs$",
      "run": "echo testing file {{file}}"
    },
    {
      "type": "testFunction",
      "file": "\\.ext$",
      "run": "echo testing file {{file}} at line {{line}}"
    }
  ]
}"#;

// creates an example config file on disk
pub fn create() -> Result<()> {
  fs::write(PATH, EXAMPLE_CONTENT)
    .map_err(|e| UserError::CannotCreateConfigFile { err: e.to_string() })
}
