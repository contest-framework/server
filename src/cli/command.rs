use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Arguments {
  #[command(subcommand)]
  command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
  /// Print the received triggers from the pipe without running them
  Debug,
  /// Run the given client-side trigger and exit
  Run {
    /// the client-side trigger to execute
    trigger: String,
  },
  /// Create an example configuration file
  Setup,
  /// Execute the received triggers from the pipe
  Start,
}

impl Command {
  /// parses the CLI args
  #[must_use]
  pub fn parse() -> Command {
    Arguments::parse().command.unwrap_or(Command::Start)
  }
}
