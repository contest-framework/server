//! commands sent over the FIFO

use super::errors::UserError;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Trigger {
    pub command: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl Trigger {
    pub fn matches_client_trigger(&self, from_client: &Trigger) -> Result<bool, UserError> {
        if self.command != from_client.command {
            return Ok(false);
        }
        if self.line.is_none() && from_client.line.is_some() {
            // client sent line but config doesn't contain it --> still a match
            return Ok(true);
        }
        if self.line != from_client.line {
            return Ok(false);
        }
        if self.file.is_none() && from_client.file.is_none() {
            return Ok(true);
        }
        if self.file.is_some() && from_client.file.is_some() {
            let self_file = &self.file.as_ref().unwrap();
            let pattern =
                glob::Pattern::new(self_file).map_err(|e| UserError::ConfigInvalidGlobPattern {
                    pattern: self_file.to_string(),
                    err: e.to_string(),
                })?;
            return Ok(pattern.matches(from_client.file.as_ref().unwrap()));
        }
        Ok(false)
    }
}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut parts: std::vec::Vec<String> = std::vec::Vec::new();
        parts.push(format!("\"command\": \"{}\"", self.command));
        if self.file.is_some() {
            parts.push(format!("\"file\": \"{}\"", self.file.as_ref().unwrap()));
        }
        if self.line.is_some() {
            parts.push(format!("\"line\": \"{}\"", self.line.as_ref().unwrap()));
        }
        write!(f, "{}", parts.join(", "))?;
        write!(f, " }}")
    }
}

pub fn from_string(line: &str) -> Result<Trigger, UserError> {
    match serde_json::from_str(line) {
        Ok(trigger) => Ok(trigger),
        Err(err) => Err(UserError::InvalidTrigger {
            line: line.to_string(),
            err: err.to_string(),
        }),
    }
}

//
// ----------------------------------------------------------------------------
//

#[cfg(test)]
mod tests {

    mod from_string {

        use super::super::*;

        #[test]
        fn test_all() {
            let give = r#"{ "command": "testAll" }"#;
            let have = from_string(give).unwrap();
            let want = Trigger {
                command: "testAll".to_string(),
                file: None,
                line: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn filename() {
            let give = r#"{ "command": "testFile", "file": "foo.rs" }"#;
            let have = from_string(give).unwrap();
            let want = Trigger {
                command: "testFile".to_string(),
                file: Some("foo.rs".to_string()),
                line: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn filename_line() {
            let give = r#"{ "command": "testFunction", "file": "foo.rs", "line": 12 }"#;
            let have = from_string(give).unwrap();
            let want = Trigger {
                command: "testFunction".to_string(),
                file: Some("foo.rs".to_string()),
                line: Some(12),
            };
            assert_eq!(have, want);
        }

        #[test]
        fn filename_extra_fields() {
            let give = r#"{ "command": "testFile", "file": "foo.rs", "other": "12" }"#;
            let have = from_string(give).unwrap();
            let want = Trigger {
                command: "testFile".to_string(),
                file: Some(String::from("foo.rs")),
                line: None,
            };
            assert_eq!(have, want);
        }

        #[test]
        fn from_string_invalid_json() {
            let give = "{\"filename}";
            let have = from_string(give);
            let want = UserError::InvalidTrigger {
                line: give.to_string(),
                err: "EOF while parsing a string at line 1 column 11".to_string(),
            };
            match have {
                Ok(_) => panic!("this shouldn't work"),
                Err(err) => assert_eq!(err, want),
            }
        }
    }

    mod matches_client_trigger {
        use super::super::*;

        #[test]
        fn matching() {
            let config = Trigger {
                command: "testFunction".to_string(),
                file: Some("**/*.rs".to_string()),
                line: Some(12),
            };
            let give = Trigger {
                command: "testFunction".to_string(),
                file: Some("foo.rs".to_string()),
                line: Some(12),
            };
            assert!(config.matches_client_trigger(&give).unwrap());
            let give = Trigger {
                command: "testFunction".to_string(),
                file: Some("foo/bar.rs".to_string()),
                line: Some(12),
            };
            assert!(config.matches_client_trigger(&give).unwrap());
        }

        #[test]
        fn mismatching_command() {
            let config = Trigger {
                command: "testFunction".to_string(),
                file: Some("filename".to_string()),
                line: Some(12),
            };
            let give = Trigger {
                command: "testFile".to_string(),
                file: Some("filename".to_string()),
                line: Some(12),
            };
            assert!(!config.matches_client_trigger(&give).unwrap());
        }

        #[test]
        fn mismatching_file() {
            let config = Trigger {
                command: "testFunction".to_string(),
                file: Some("filename".to_string()),
                line: Some(12),
            };
            let give = Trigger {
                command: "testFile".to_string(),
                file: Some("filename2".to_string()),
                line: Some(12),
            };
            assert!(!config.matches_client_trigger(&give).unwrap());
        }

        #[test]
        fn mismatching_line() {
            let config = Trigger {
                command: "testFunction".to_string(),
                file: Some("filename".to_string()),
                line: Some(12),
            };
            let give = Trigger {
                command: "testFile".to_string(),
                file: Some("filename".to_string()),
                line: Some(11),
            };
            assert!(!config.matches_client_trigger(&give).unwrap());
        }
    }
}
