use contest::cli::Command;
use contest::{Result, cli, client, config, listen, run_with_decoration};
use std::env;
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
  let current_dir = env::current_dir().unwrap_or_else(|err| cli::exit(&err.to_string()));
  let _ = client::fifo::in_dir(&current_dir).delete();
  exit_code
}

fn main_with_result() -> Result<()> {
  let config = config::file::read()?;
  match Command::parse() {
    Command::Start => listen(&config, false),
    Command::Debug => listen(&config, true),
    Command::Run { trigger } => {
      println!("running trigger: {trigger}");
      let mut last_command: Option<String> = None;
      run_with_decoration(&trigger, &config, false, &mut last_command)
    }
    Command::Setup => config::file::create(),
  }
}
