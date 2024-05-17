use clap::{Parser, Subcommand};
use std::env;
use tertestrial::{cli, client, config, listen, run_with_decoration, Result};

fn main() {
  if let Err(err) = main_with_result() {
    let (msg, guidance) = err.messages();
    println!("\nError: {msg}\n\n{guidance}");
  }
  let current_dir = match env::current_dir() {
    Ok(dir) => dir,
    Err(err) => cli::exit(&err.to_string()),
  };
  let _ = client::fifo::in_dir(&current_dir).delete();
}

fn main_with_result() -> Result<()> {
  match Cli::parse().command.unwrap_or(Command::Start) {
    Command::Start => listen(false),
    Command::Debug => listen(true),
    Command::Run { trigger } => {
      println!("running trigger: {trigger}");
      let config = config::file::read()?;
      let mut last_command: Option<String> = None;
      run_with_decoration(&trigger, &config, &mut last_command)
    }
    Command::Setup => config::file::create(),
  }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
  #[command(subcommand)]
  command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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
