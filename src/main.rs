use std::{env, panic};
use tertestrial::cli::{self, Command};
use tertestrial::{client, config, listen, run_with_decoration, Result};

fn main() {
  let panic_result = panic::catch_unwind(|| {
    if let Err(err) = main_with_result() {
      let (msg, guidance) = err.messages();
      println!("\nError: {}\n\n{}", msg, guidance);
    }
  });
  let _ = client::fifo::in_dir(&env::current_dir().unwrap()).delete();
  if panic_result.is_err() {
    panic!("{:?}", panic_result);
  }
}

fn main_with_result() -> Result<()> {
  match cli::args::parse(env::args())? {
    Command::Normal => listen(false),
    Command::Debug => listen(true),
    Command::Run(cmd) => {
      println!("running cmd: {}", cmd);
      let config = config::file::read()?;
      let mut last_command: Option<String> = None;
      run_with_decoration(cmd, &config, &mut last_command)
    }
    Command::Setup => config::file::create(),
  }
}
