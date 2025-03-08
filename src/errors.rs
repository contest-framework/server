//! error types used in this app

use big_s::S;

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
  MissingFileAndLineInTrigger,
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
        S("This could mean a Contest instance could already be running.\nIf you are sure no other instance is running, please delete this file and start Contest again."),
      ),
      UserError::FifoCannotCreate { err, path } => (format!("Cannot create pipe at {path}: {err}"), String::new()),
      UserError::FifoCannotDelete { err, path } => (format!("Cannot delete pipe at {path}: {err}"), String::new()),
      UserError::FifoCannotRead { err } => (format!("Cannot read from pipe: {err}"), S("This is an internal error")),
      UserError::FifoCannotOpen { err } => (format!("Cannot open pipe: {err}"), S("This is an internal error")),
      UserError::FileNameNotAvailable => (
        S("Filename is not known"),
        S(r#"To use the filename in a variable, you need to choose either the "testFile" or "testFileLine" action type that provides this data."#),
      ),
      UserError::FilesIsEmpty => (S(r#"The "files" field in your config file is empty"#), String::new()),
      UserError::InvalidRegex { regex, err } => (format!("invalid regex: {regex}"), err.to_string()),
      UserError::InvalidTrigger { source: line, err } => (format!("cannot parse command received from client: {line}"), err.to_owned()),
      UserError::LineIsNotANumber { line } => (format!("the provided line ({line})is not a number"), String::new()),
      UserError::LineNotAvailable => (
        S("Line not available"),
        S(r#"To use the current line in a variable, you need to use the "testFileLine" action type that provides this data."#),
      ),
      UserError::MissingFilesInPattern => (S(r#"the pattern in the config file is missing the "files" field."#), String::new()),
      UserError::MissingFileInTrigger => (S(r#"the trigger received from the client is missing the "file" field."#), String::new()),
      UserError::MissingFileAndLineInTrigger => (S(r#"the trigger received from the client is missing the "file" and "line" fields."#), String::new()),
      UserError::MissingFilesInTestFile => (S(r#"missing "files" entry in "testFile" action"#), String::new()),
      UserError::MissingLineInTrigger => (S(r#"the trigger received from the client is missing the "line" field."#), String::new()),
      UserError::MissingRunInTrigger => (S(r#"missing "run" entry in "customCommand" trigger"#), String::new()),
      UserError::NoCommandToRepeat {} => (S("No command to repeat found"), S("You must submit a test command first before you can repeat it.")),
      UserError::RunCommandNotFound { command } => (
        format!("test command to run not found: {command}"),
        S("Please verify that the command is in the path or fix your config file."),
      ),
      UserError::RunCommandIsEmpty => (S(r#"the "run" field in your configuration file is empty"#), String::new()),
      UserError::TriggerTooManyCaptures { count, regex, line } => (
        format!("found {count} captures using regex \"{regex}\" on line: {line}"),
        S("filters in the Contest configuration file can only contain one capture group"),
      ),
      UserError::TriggerRegexNotFound { regex, filename, line } => (
        format!("did not find pattern {regex} in file {filename} at line {line}"),
        S("This is defined in file .testconfig.json."),
      ),
      UserError::UnknownActionType { action_type } => (
        format!("unknown action type: {action_type}"),
        S(r#"Valid types are "testAll", "testFile", and "testFileLine"."#),
      ),
      UserError::UnknownTrigger { source } => (
        format!("cannot determine command for trigger: {source}"),
        S("Please make sure that this action is listed in your configuration file"),
      ),
    }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = std::result::Result<T, UserError>;
