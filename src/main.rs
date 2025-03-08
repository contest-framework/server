use contest::cli::Command;
use contest::client::fifo;
use contest::{Result, config, listen, run_with_decoration};
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
  match Command::parse() {
    Command::Start => listen(false),
    Command::Debug => listen(true),
    Command::Run { trigger } => {
      println!("running trigger: {trigger}");
      let config = config::file::read()?;
      let mut last_command: Option<String> = None;
      run_with_decoration(&trigger, &config, false, &mut last_command)
    }
    Command::Setup => config::file::create(),
  }
}
