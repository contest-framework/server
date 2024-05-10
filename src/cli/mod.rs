pub mod args;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Normal,      // normal operation
    Debug,       // print the received commands from the pipe
    Run(String), // run the given command manually
    Setup,       // create a config file
}
