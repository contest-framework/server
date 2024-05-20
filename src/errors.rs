//! error types used in this app

/// The possible errors that the user can cause and needs to be notified about.
#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
  CannotCreateConfigFile { err: String },
  CannotDetermineCurrentDirectory { err: String },
  CannotReadFile { path: String, err: String },
  CannotSplitShellString { source: String, err: String },
  ConfigFileError { err: String },
  ConfigFileInvalidContent { err: String },
  ConfigInvalidGlob { pattern: String, err: String },
  FifoAlreadyExists { path: String },
  FifoCannotCreate { path: String, err: String },
  FifoCannotDelete { path: String, err: String },
  FifoCannotOpen { err: String },
  FifoCannotRead { err: String },
  FilesIsEmpty,
  FileNameNotAvailable,
  InvalidRegex { regex: String, err: String },
  InvalidTrigger { source: String, err: String },
  LineIsNotANumber { line: String },
  LineNotAvailable,
  MissingFilesInPattern,
  MissingFileInTrigger,
  MissingFilesInTestFile,
  MissingLineInTrigger,
  MissingRunInTrigger,
  NoCommandToRepeat,
  RunCommandNotFound { command: String },
  RunCommandIsEmpty,
  TriggerTooManyCaptures { count: usize, regex: String, line: String },
  TriggerRegexNotFound { regex: String, filename: String, line: usize },
  UnknownActionType { action_type: String },
  UnknownTrigger { source: String },
}

impl UserError {
  /// Provides human-readable messages for `UserError`.
  #[must_use]
  pub fn messages(&self) -> (String, String) {
    match self {
      UserError::CannotCreateConfigFile { err } => (format!("cannot create configuration file: {err}"), String::new()),
      UserError::CannotDetermineCurrentDirectory { err } => (format!("cannot determine the current directory: {err}"), String::new()),
      UserError::CannotReadFile { path, err } => (format!("cannot read file {path}"), err.into()),
      UserError::CannotSplitShellString { source, err } => (format!("cannot split this shell string: {source}"), err.into()),
      UserError::ConfigFileInvalidContent { err } => (format!("Cannot parse configuration file: {err}"), String::new()),
      UserError::ConfigFileError { err } => (format!("Cannot open configuration file: {err}"), String::new()),
      UserError::ConfigInvalidGlob { pattern, err } => (format!("Invalid glob pattern: {pattern}"), err.into()),
      UserError::FifoAlreadyExists { path } => (
        format!("A fifo pipe \"{path}\" already exists."),
        "This could mean a Tertestrial instance could already be running.\nIf you are sure no other instance is running, please delete this file and start Tertestrial again."
          .into(),
      ),
      UserError::FifoCannotCreate { err, path } => (format!("Cannot create pipe at {path}: {err}"), String::new()),
      UserError::FifoCannotDelete { err, path } => (format!("Cannot delete pipe at {path}: {err}"), String::new()),
      UserError::FifoCannotRead { err } => (format!("Cannot read from pipe: {err}"), "This is an internal error".into()),
      UserError::FifoCannotOpen { err } => (format!("Cannot open pipe: {err}"), "This is an internal error".into()),
      UserError::FileNameNotAvailable => (
        "Filename is not known".into(),
        r#"To use the filename in a variable, you need to choose either the "testFile" or "testFileLine" action type that provides this data."#.into(),
      ),
      UserError::FilesIsEmpty => (r#"The "files" field in your config file is empty"#.into(), String::new()),
      UserError::InvalidRegex { regex, err } => (format!("invalid regex: {regex}"), err.to_string()),
      UserError::InvalidTrigger { source: line, err } => (format!("cannot parse command received from client: {line}"), err.to_owned()),
      UserError::LineIsNotANumber { line } => (format!("the provided line ({line})is not a number"), String::new()),
      UserError::LineNotAvailable => (
        "Line not available".into(),
        r#"To use the current line in a variable, you need to use the "testFileLine" action type that provides this data."#.into(),
      ),
      UserError::MissingFilesInPattern => (r#"the pattern in the config file is missing the "files" field."#.into(), String::new()),
      UserError::MissingFileInTrigger => (r#"the trigger received from the client is missing the "file" field."#.into(), String::new()),
      UserError::MissingFilesInTestFile => (r#"missing "files" entry in "testFile" action"#.into(), String::new()),
      UserError::MissingLineInTrigger => (r#"the trigger received from the client is missing the "line" field."#.into(), String::new()),
      UserError::MissingRunInTrigger => (r#"missing "run" entry in "customCommand" trigger"#.into(), String::new()),
      UserError::NoCommandToRepeat {} => ("No command to repeat found".into(), "You must submit a test command first before you can repeat it.".into()),
      UserError::RunCommandNotFound { command } => (
        format!("test command to run not found: {command}"),
        "Please verify that the command is in the path or fix your config file.".into(),
      ),
      UserError::RunCommandIsEmpty => (r#"the "run" field in your configuration file is empty"#.into(), String::new()),
      UserError::TriggerTooManyCaptures { count, regex, line } => (
        format!("found {count} captures using regex \"{regex}\" on line: {line}"),
        "filters in the Tertestrial configuration file can only contain one capture group".into(),
      ),
      UserError::TriggerRegexNotFound { regex, filename, line } => (
        format!("did not find pattern {regex} in file {filename} at line {line}"),
        "This is defined in file .testconfig.json.".into(),
      ),
      UserError::UnknownActionType { action_type } => (
        format!("unknown action type: {action_type}"),
        r#"Valid types are "testAll", "testFile", and "testFileLine"."#.into(),
      ),
      UserError::UnknownTrigger { source } => (
        format!("cannot determine command for trigger: {source}"),
        "Please make sure that this action is listed in your configuration file".into(),
      ),
    }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = std::result::Result<T, UserError>;
