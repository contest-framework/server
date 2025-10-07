use contest::cli::Command;
use contest::client::fifo;
use contest::config::Configuration;
use contest::{Result, listen, run_with_decoration};
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
  let mut exit_code = ExitCode::SUCCESS;
  if let Err(err) = main_with_result() {
    match err.messages() {
      (msg, Some(guidance)) => println!("\nError: {msg}\n\n{guidance}"),
      (msg, None) => println!("\nError: {msg}"),
    }
    exit_code = ExitCode::FAILURE;
  }
  let _ = fs::remove_file(fifo::FILE_NAME);
  exit_code
}

fn main_with_result() -> Result<()> {
  let config = Configuration::read()?;
  match Command::parse() {
    Command::Start => listen(&config, false),
    Command::Debug => listen(&config, true),
    Command::Run { trigger } => {
      println!("running trigger: {trigger}");
      let mut last_command: Option<String> = None;
      let _ = run_with_decoration(trigger, &config, false, &mut last_command)?;
      Ok(())
    }
    Command::Init => Configuration::create(),
  }
}
