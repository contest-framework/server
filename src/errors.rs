//! error types used in this app

/// The possible errors that the user can cause and needs to be notified about.
#[derive(Debug, PartialEq)]
pub enum UserError {
    CannotCreateConfigFile {
        err: String,
    },
    ConfigFileNotFound {},
    ConfigFileNotReadable {
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
    FifoCannotDelete {
        err: String,
    },
    FifoCannotRead {
        err: String,
    },
    InvalidTrigger {
        line: String,
        err: String,
    },
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
    },
    UnknownTrigger {
        line: String,
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
            UserError::ConfigFileNotReadable{err} => (format!("Cannot open configuration file: {}", err), "".into()),
            UserError::ConfigInvalidGlobPattern{pattern, err} => (format!("Invalid glob pattern: {}", pattern), err.into()),
            UserError::FifoAlreadyExists{path} => (format!("A fifo pipe \"{}\" already exists.", path), "This could mean a Tertestrial instance could already be running.\nIf you are sure no other instance is running, please delete this file and start Tertestrial again.".into()),
            UserError::FifoCannotDelete{err} => (format!("Cannot delete pipe: {}", err), "".into()),
            UserError::FifoCannotRead{err} => (format!("Cannot read from pipe: {}", err), "This is an internal error".into()),
            UserError::InvalidTrigger{line, err} => (format!("cannot parse command received from client: {}", line),
                format!( "This is a problem with your Tertestrial client.\n\nError message from JSON parser: {}", err)),
            UserError::NoCommandToRepeat{} => ("No command to repeat found".into(), "You must submit a test command first before you can repeat it.".into()),
            UserError::RunCommandNotFound{command} => (format!("test command to run not found: {}", command),
                        "Please verify that the command is in the path or fix your config file.".into()),
            UserError::TriggerTooManyCaptures{count, regex, line} => (format!("found {} captures using regex \"{}\" on line: {}", count, regex, line),
                    "filters in the Tertestrial configuration file can only contain one capture group".into()),
            UserError::TriggerRegexNotFound{regex, filename} => (format!("Did not find pattern {} in file {}", regex, filename),
                "Please check that the file .testconfig.json is correct".into()),
            UserError::UnknownTrigger{line} => (format!("cannot determine command for trigger: {}", line),
            "Please make sure that this trigger is listed in your configuration file".into()),
        }
    }
}
