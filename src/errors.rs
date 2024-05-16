//! error types used in this app

/// The possible errors that the user can cause and needs to be notified about.
#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
  CannotCreateConfigFile {
    err: String,
  },
  ConfigFileNotFound {},
  ConfigFileError {
    err: String,
  },
  ConfigFileInvalidContent {
    err: String,
  },
  ConfigInvalidGlobPattern {
    pattern: String,
    err: String,
  },
  FifoAlreadyExists {
    path: String,
  },
  FifoCannotCreate {
    path: String,
    err: String,
  },
  FifoCannotDelete {
    path: String,
    err: String,
  },
  FifoCannotRead {
    err: String,
  },
  InvalidRegex {
    regex: String,
    err: String,
  },
  InvalidTrigger {
    source: String,
    err: String,
  },
  MissingFilesInPattern,
  MissingFileInTrigger,
  MissingFilesInTestFile,
  MissingLineFieldInCurrentOrAboveLineContent,
  MissingLineInTrigger,
  NoCommandToRepeat {},
  RunCommandNotFound {
    command: String,
  },
  TriggerTooManyCaptures {
    count: usize,
    regex: String,
    line: String,
  },
  TriggerRegexNotFound {
    regex: String,
    filename: String,
    line: u32,
  },
  UnknownActionType {
    action_type: String,
  },
  UnknownTrigger {
    source: String,
  },
}

impl UserError {
  /// Provides human-readable messages for TertErrors.
  pub fn messages(&self) -> (String, String) {
    match self {
            UserError::CannotCreateConfigFile{err} => (format!("cannot create configuration file: {}", err), "".into()),
            UserError::ConfigFileInvalidContent{err} => {
                (format!("Cannot parse configuration file: {}", err), "".into())
            }
            UserError::ConfigFileNotFound{} => ("Configuration file not found".into(), "Tertestrial requires a configuration file named \".testconfig.json\" in the current directory. Please run \"tertestrial setup \" to create one.".into()),
            UserError::ConfigFileError{err} => (format!("Cannot open configuration file: {}", err), "".into()),
            UserError::ConfigInvalidGlobPattern{pattern, err} => (format!("Invalid glob pattern: {}", pattern), err.into()),
            UserError::FifoAlreadyExists{path} => (format!("A fifo pipe \"{}\" already exists.", path), "This could mean a Tertestrial instance could already be running.\nIf you are sure no other instance is running, please delete this file and start Tertestrial again.".into()),
            UserError::FifoCannotCreate { err, path } => (format!("Cannot create pipe at {path}: {err}"), "".into()),
            UserError::FifoCannotDelete{err, path} => (format!("Cannot delete pipe at {path}: {err}"), "".into()),
            UserError::FifoCannotRead{err} => (format!("Cannot read from pipe: {}", err), "This is an internal error".into()),
            UserError::InvalidRegex { regex, err } => (format!("invalid regex: {regex}"), err.to_string()),
            UserError::InvalidTrigger{source: line, err} => (format!("cannot parse command received from client: {}", line), err.to_owned()),
            UserError::MissingFilesInPattern  => (r#"the pattern in the config file is missing the "files" field."#.into(), "".into()),
            UserError::MissingFileInTrigger  => (r#"the trigger received from the client is missing the "file" field."#.into(), "".into()),
            UserError::MissingFilesInTestFile => (r#"missing "files" entry in "testFile" action"#.into(), "".into()),
            UserError::MissingLineFieldInCurrentOrAboveLineContent => ("missing \"line\" field".into(), "".into()),
            UserError::MissingLineInTrigger  => (r#"the trigger received from the client is missing the "line" field."#.into(), "".into()),
            UserError::NoCommandToRepeat{} => ("No command to repeat found".into(), "You must submit a test command first before you can repeat it.".into()),
            UserError::RunCommandNotFound{command} => (format!("test command to run not found: {}", command),
                        "Please verify that the command is in the path or fix your config file.".into()),
            UserError::TriggerTooManyCaptures{count, regex, line} => (format!("found {} captures using regex \"{}\" on line: {}", count, regex, line),
                    "filters in the Tertestrial configuration file can only contain one capture group".into()),
            UserError::TriggerRegexNotFound{regex, filename, line } => (format!("Did not find pattern {} in file {} at line {}", regex, filename, line),
                "Please check that the file .testconfig.json is correct".into()),
            UserError::UnknownActionType { action_type } => (format!("unknown action type: {action_type}"), r#"Valid types are "testAll", "testFile", and "testFunction"."#.into()),
            UserError::UnknownTrigger{source } => (format!("cannot determine command for trigger: {}", source),
            "Please make sure that this action is listed in your configuration file".into()),
        }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = std::result::Result<T, UserError>;
