//! error types used in this app

/// The possible errors that the user can cause and needs to be notified about.
#[derive(Debug, PartialEq)]
pub enum UserError {
    ArgsMissingOptionForRunCommand {},
    ArgsUnknownCommand {
        command: String,
    },
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
            UserError::ArgsMissingOptionForRunCommand{} => ("missing option for \"run\" command".to_string(), "The \"run\" command requires the command to run".to_string()),
            UserError::ArgsUnknownCommand { command }  => (format!("unknown argument: {}", command), help()),
            UserError::CannotCreateConfigFile{err} => (format!("cannot create configuration file: {}", err), "".to_string()),
            UserError::ConfigFileInvalidContent{err} => {
                (format!("Cannot parse configuration file: {}", err), "".to_string())
            }
            UserError::ConfigFileNotFound{} => ("Configuration file not found".to_string(), "Tertestrial requires a configuration file named \".testconfig.json\" in the current directory. Please run \"tertestrial setup \" to create one.".to_string()),
            UserError::ConfigFileNotReadable{err} => (format!("Cannot open configuration file: {}", err), "".to_string()),
            UserError::ConfigInvalidGlobPattern{pattern, err} => (format!("Invalid glob pattern: {}", pattern), err.to_string()),
            UserError::FifoAlreadyExists{path} => (format!("A fifo pipe \"{}\" already exists.", path), "This could mean a Tertestrial instance could already be running.\nIf you are sure no other instance is running, please delete this file and start Tertestrial again.".to_string()),
            UserError::FifoCannotDelete{err} => (format!("Cannot delete pipe: {}", err), "".to_string()),
            UserError::FifoCannotRead{err} => (format!("Cannot read from pipe: {}", err), "This is an internal error".to_string()),
            UserError::InvalidTrigger{line, err} => (format!("cannot parse command received from client: {}", line),
                format!( "This is a problem with your Tertestrial client.\n\nError message from JSON parser: {}", err)),
            UserError::NoCommandToRepeat{} => ("No command to repeat found".to_string(), "You must submit a test command first before you can repeat it.".to_string()),
            UserError::RunCommandNotFound{command} => (format!("test command to run not found: {}", command),
                        "Please verify that the command is in the path or fix your config file.".to_string()),
            UserError::TriggerTooManyCaptures{count, regex, line} => (format!("found {} captures using regex \"{}\" on line: {}", count, regex, line),
                    "filters in the Tertestrial configuration file can only contain one capture group".to_string()),
            UserError::TriggerRegexNotFound{regex, filename} => (format!("Did not find pattern {} in file {}", regex, filename),
                "Please check that the file .testconfig.json is correct".to_string()),
            UserError::UnknownTrigger{line} => (format!("cannot determine command for trigger: {}", line),
            "Please make sure that this trigger is listed in your configuration file".to_string()),
        }
    }
}

pub fn help() -> String {
    "\
Usage: tertestial [command]

Without command, Tertestrial starts normally.

You can provide one of the following commands:
- debug: prints the commands received from the client without running them
- help: display this help screen
- run: executes the given command as if it was received by the client
- setup: create an example configuration file
- version: show the version of the installed Tertestrial server
"
    .to_string()
}
