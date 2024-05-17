mod channel;
pub mod cli;
pub mod client;
pub mod config;
mod errors;
pub mod scanner;
mod subshell;
pub mod template;

use client::{fifo, Trigger};
pub use errors::{Result, UserError};
use std::env;
use std::io::Write;
use subshell::Outcome;
use termcolor::WriteColor;
use terminal_size::terminal_size;

pub fn listen(debug: bool) -> Result<()> {
  let config = config::file::read()?;
  if debug {
    println!("using this configuration:");
    println!("{config}");
  }
  let (sender, receiver) = channel::create(); // cross-thread communication channel
  cli::ctrl_c::handle(sender.clone());
  let current_dir =
    env::current_dir().map_err(|err| UserError::CannotDetermineCurrentDirectory {
      err: err.to_string(),
    })?;
  let pipe = client::fifo::in_dir(&current_dir);
  pipe.create()?;
  fifo::listen(pipe, sender);
  let mut last_command: Option<String> = None;
  if debug {
    println!("Tertestrial is online in debug mode, Ctrl-C to exit");
  } else {
    println!("Tertestrial is online, Ctrl-C to exit");
  }
  for signal in receiver {
    match signal {
      channel::Signal::ReceivedLine(line) if debug => println!("received from client: {line}"),
      channel::Signal::ReceivedLine(line) => {
        run_with_decoration(&line, &config, &mut last_command)?;
      }
      channel::Signal::Exit => {
        println!("\nSee you later!");
        return Ok(());
      }
    }
  }
  Ok(())
}

pub fn run_with_decoration(
  text: &str,
  config: &config::Configuration,
  last_command: &mut Option<String>,
) -> Result<()> {
  for _ in 0..config.options.before_run.newlines {
    println!();
  }
  if config.options.before_run.clear_screen {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  }
  let result = run_command(text, config, last_command)?;
  for _ in 0..config.options.after_run.newlines {
    println!();
  }
  match terminal_size() {
    None => eprintln!("Warning: cannot determine terminal size"),
    Some((width, _)) => {
      for _ in 0..config.options.after_run.indicator_lines {
        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        let color = if result {
          termcolor::Color::Green
        } else {
          termcolor::Color::Red
        };
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(color)));
        let text: String = "█".repeat(width.0 as usize);
        writeln!(&mut stdout, "{text}").unwrap();
        let _ = stdout.reset(); // we really don't care about being unable to reset colors here
      }
    }
  }
  Ok(())
}

fn run_command(
  text: &str,
  configuration: &config::Configuration,
  last_command: &mut Option<String>,
) -> Result<bool> {
  let trigger = Trigger::parse(text)?;
  match configuration.get_command(&trigger, last_command) {
    Err(err) => match err {
      UserError::NoCommandToRepeat {} => {
        // repeat non-existing command --> don't stop, just print an error message and keep going
        let (msg, desc) = err.messages();
        println!("{msg}");
        println!("{desc}");
        Ok(false)
      }
      _ => Err(err),
    },
    Ok(command) => match subshell::run(&command)? {
      Outcome::TestPass() => {
        println!("SUCCESS!");
        Ok(true)
      }
      Outcome::TestFail() => {
        println!("FAILED!");
        Ok(false)
      }
      Outcome::NotFound(command) => Err(UserError::RunCommandNotFound { command }),
    },
  }
}
