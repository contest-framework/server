mod channel;
pub mod cli;
pub mod client;
pub mod config;
mod errors;
pub mod scanner;
mod subshell;
pub mod template;

use client::{Trigger, fifo};
pub use errors::{Result, UserError};
use std::env;
use std::io::Write;
use subshell::Outcome;
use termcolor::WriteColor;
use terminal_size::{Height, Width, terminal_size};

pub fn listen(debug: bool) -> Result<()> {
  let config = config::file::read()?;
  if debug {
    println!("using this configuration:");
    println!("{config}");
  }
  let (sender, receiver) = channel::create(); // cross-thread communication channel
  cli::ctrl_c::handle(sender.clone());
  let current_dir = env::current_dir().map_err(|err| UserError::CannotDetermineCurrentDirectory { err: err.to_string() })?;
  let pipe = client::fifo::in_dir(&current_dir);
  pipe.listen(sender)?;
  let mut last_command: Option<String> = None;
  if debug {
    println!("Contest is online in debug mode, Ctrl-C to exit");
  } else {
    println!("Contest is online, Ctrl-C to exit");
  }
  for signal in receiver {
    match signal {
      channel::Signal::ReceivedLine(line) => run_with_decoration(&line, &config, debug, &mut last_command)?,
      channel::Signal::Exit => {
        println!("\nSee you later!");
        return Ok(());
      }
    }
  }
  Ok(())
}

pub fn run_with_decoration(text: &str, config: &config::Configuration, debug: bool, last_command: &mut Option<String>) -> Result<()> {
  if debug {
    println!("received from client: {text}");
    return Ok(());
  }
  for _ in 0..config.options.before_run.newlines {
    println!();
  }
  if config.options.before_run.clear_screen {
    print!("{esc}[2J{esc}[1;1H{esc}c", esc = 27 as char);
  }
  let result = run_command(text, config, last_command)?;
  for _ in 0..config.options.after_run.newlines {
    println!();
  }
  let terminal_width = terminal_size().unwrap_or((Width(80), Height(20))).0;
  for _ in 0..config.options.after_run.indicator_lines {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
    let color = cli::error_color(result);
    let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(color)));
    let text: String = "█".repeat(terminal_width.0 as usize);
    writeln!(&mut stdout, "{text}").unwrap();
    let _ = stdout.reset(); // we really don't care about being unable to reset colors here
  }
  Ok(())
}

fn run_command(text: &str, configuration: &config::Configuration, last_command: &mut Option<String>) -> Result<bool> {
  let trigger = Trigger::try_from(text)?;
  let command = match configuration.get_command(&trigger, last_command) {
    Err(err) => match err {
      UserError::NoCommandToRepeat => {
        // repeat non-existing command --> don't stop, just print an error message and keep going
        cli::print_error(&err);
        return Ok(false);
      }
      UserError::TriggerRegexNotFound { regex: _, filename: _, line: _ } => {
        // user triggered a command in a place where it doesn't match all regexes --> let them know and go to the correct location
        cli::print_error(&err);
        return Ok(false);
      }
      UserError::UnknownTrigger { source: _ } => {
        // user sent a trigger from the wrong file --> let them know and send one from the correct file
        cli::print_error(&err);
        return Ok(false);
      }
      _ => return Err(err),
    },
    Ok(command) => command,
  };
  last_command.replace(command.clone());
  match subshell::run(&command)? {
    Outcome::TestPass() => {
      if configuration.options.after_run.print_result {
        println!("SUCCESS");
      }
      Ok(true)
    }
    Outcome::TestFail() => {
      println!("FAILED");
      Ok(false)
    }
    Outcome::NotFound(command) => Err(UserError::RunCommandNotFound { command }),
  }
}
