use clap::{Parser, Subcommand};
use std::{env, panic};
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
  let args = Args::parse();
  match args.command {
    None => listen(false),
    Some(command) => match command {
      Command::Start => listen(false),
      Command::Debug => listen(true),
      Command::Run { trigger } => {
        println!("running trigger: {}", trigger);
        let config = config::file::read()?;
        let mut last_command: Option<String> = None;
        run_with_decoration(trigger, &config, &mut last_command)
      }
      Command::Setup => config::file::create(),
    },
  }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
  #[command(subcommand)]
  command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
  Debug,
  Run { trigger: String },
  Setup,
  Start,
}
