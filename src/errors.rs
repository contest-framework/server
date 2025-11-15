//! error types used in this app

use crate::config;
use crate::config::file::ActionType;
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
  MissingFileInTrigger { original: String },
  MissingFileAndLineInTrigger { original: String },
  MissingFilesInTestFile { original: String },
  MissingLineInTrigger { original: String },
  MissingRunInTrigger { original: String },
  NoCommandToRepeat,
  RunCommandNotFound { command: String },
  RunCommandIsEmpty,
  TriggerTooManyCaptures { count: usize, regex: String, line: String },
  TriggerRegexNotFound { regex: String, filename: String, line: usize },
  UnknownActionType { action_type: ActionType },
  UnknownTrigger { source: String },
}

impl UserError {
  /// Provides human-readable messages for `UserError`.
  #[must_use]
  pub fn messages(self) -> (String, Option<String>) {
    match self {
      UserError::CannotCreateConfigFile { err } => (format!("cannot create configuration file: {err}"), None),
      UserError::CannotDetermineCurrentDirectory { err } => (format!("cannot determine the current directory: {err}"), None),
      UserError::CannotReadFile { path, err } => (format!("cannot read file {path}"), Some(err)),
      UserError::CannotSplitShellString { source, err } => (format!("cannot split this shell string: {source}"), Some(err)),
      UserError::ConfigFileInvalidContent { err } => (format!("Cannot parse configuration file: {err}"), None),
      UserError::ConfigFileError { err } => (format!("Cannot open configuration file: {err}"), None),
      UserError::ConfigInvalidGlob { pattern, err } => (format!("Invalid glob pattern: {pattern}"), Some(err)),
      UserError::FifoAlreadyExists { path } => (
        format!("A fifo pipe \"{path}\" already exists."),
        Some(S(
          "This could mean a Contest instance could already be running.\nIf you are sure no other instance is running, please delete this file and start Contest again.",
        )),
      ),
      UserError::FifoCannotCreate { err, path } => (format!("Cannot create pipe at {path}: {err}"), None),
      UserError::FifoCannotDelete { err, path } => (format!("Cannot delete pipe at {path}: {err}"), None),
      UserError::FifoCannotRead { err } => (format!("Cannot read from pipe: {err}"), Some(S("This is an internal error"))),
      UserError::FifoCannotOpen { err } => (format!("Cannot open pipe: {err}"), Some(S("This is an internal error"))),
      UserError::FileNameNotAvailable => (
        S("Filename is not known"),
        Some(S(
          r#"To use the filename in a variable, you need to choose either the "test-file" or "test-file-line" action type that provides this data."#,
        )),
      ),
      UserError::FilesIsEmpty => (S(r#"The "files" field in your config file is empty"#), None),
      UserError::InvalidRegex { regex, err } => (format!("invalid regex: {regex}"), Some(err)),
      UserError::InvalidTrigger { source: line, err } => (format!("cannot parse command received from client: {line}"), Some(err)),
      UserError::LineIsNotANumber { line } => (format!("the provided line ({line})is not a number"), None),
      UserError::LineNotAvailable => (
        S("Line not available"),
        Some(S(
          r#"To use the current line in a variable, you need to use the "test-file-line" action type that provides this data."#,
        )),
      ),
      UserError::MissingFilesInPattern => (S(r#"the pattern in the config file is missing the "files" field."#), None),
      UserError::MissingFileInTrigger { original } => (format!("invalid trigger received: {original}"), Some(S(r#"missing "file" field"#))),
      UserError::MissingFileAndLineInTrigger { original } => (format!("invalid trigger received: {original}"), Some(S(r#"missing "file" and "line" fields"#))),
      UserError::MissingFilesInTestFile { original } => (format!("invalid trigger received: {original}"), Some(S(r#"missing "files" field"#))),
      UserError::MissingLineInTrigger { original } => (format!("invalid trigger received: {original}"), Some(S(r#"missing "line" field"#))),
      UserError::MissingRunInTrigger { original } => (format!("invalid trigger received: {original}"), Some(S(r#"missing "run" field"#))),
      UserError::NoCommandToRepeat => (S("No command to repeat found"), Some(S("You must submit a test command first before you can repeat it."))),
      UserError::RunCommandNotFound { command } => (
        format!("test command to run not found: {command}"),
        Some(S("Please verify that the command is in the path or fix your config file.")),
      ),
      UserError::RunCommandIsEmpty => (S(r#"the "run" field in your configuration file is empty"#), None),
      UserError::TriggerTooManyCaptures { count, regex, line } => (
        format!("found {count} captures using regex \"{regex}\" on line: {line}"),
        Some(S("filters in the Contest configuration file can only contain one capture group")),
      ),
      UserError::TriggerRegexNotFound { regex, filename, line } => (
        format!("did not find pattern {regex} in file {filename} at line {line}"),
        Some(S("This is defined in file contest.json.")),
      ),
      UserError::UnknownActionType { action_type } => (
        format!("unknown action type: {action_type}"),
        Some(S(r#"Valid types are "test-all", "test-file", and "test-file-line"."#)),
      ),
      UserError::UnknownTrigger { source } => (
        format!("cannot determine command for trigger: {source}"),
        Some(format!("Please make sure that this action is listed in {}", config::JSON_PATH)),
      ),
    }
  }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = std::result::Result<T, UserError>;
