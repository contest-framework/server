#[macro_use]
extern crate prettytable;

mod channel;
mod cli;
mod config;
mod ctrl_c;
mod errors;
mod fifo;
mod run;
mod trigger;

use cli::Command;
pub use errors::{Result, UserError};
use run::Outcome;
use std::io::Write;
use std::{env, panic};
use termcolor::WriteColor;
use terminal_size::terminal_size;
pub use trigger::Trigger;

fn main() {
    let panic_result = panic::catch_unwind(|| {
        if let Err(err) = main_with_result() {
            let (msg, guidance) = err.messages();
            println!("\nError: {}\n\n{}", msg, guidance);
        }
    });
    let _ = fifo::in_dir(&env::current_dir().unwrap()).delete();
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
            run_with_decoration(cmd, &config)
        }
        Command::Setup => config::file::create(),
    }
}

fn listen(debug: bool) -> Result<()> {
    let config = config::file::read()?;
    if debug {
        println!("using this configuration:");
        println!("{}", config);
    }
    let (sender, receiver) = channel::create(); // cross-thread communication channel
    ctrl_c::handle(sender.clone());
    let pipe = fifo::in_dir(&env::current_dir().unwrap());
    match pipe.create() {
        fifo::CreateOutcome::AlreadyExists(path) => {
            return Err(UserError::FifoAlreadyExists { path })
        }
        fifo::CreateOutcome::OtherError(err) => panic!("{}", err),
        fifo::CreateOutcome::Ok() => {}
    }
    fifo::listen(pipe, sender);
    match debug {
        false => println!("Tertestrial is online, Ctrl-C to exit"),
        true => println!("Tertestrial is online in debug mode, Ctrl-C to exit"),
    }
    for signal in receiver {
        match signal {
            channel::Signal::ReceivedLine(line) => match debug {
                true => println!("received from client: {}", line),
                false => run_with_decoration(line, &config)?,
            },
            channel::Signal::CannotReadPipe(err) => {
                return Err(UserError::FifoCannotRead {
                    err: err.to_string(),
                })
            }
            channel::Signal::Exit => {
                println!("\nSee you later!");
                return Ok(());
            }
        }
    }
    Ok(())
}

fn run_with_decoration(text: String, config: &config::Configuration) -> Result<()> {
    for _ in 0..config.options.before_run.newlines {
        println!();
    }
    if config.options.before_run.clear_screen {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    let result = run_command(text, config)?;
    for _ in 0..config.options.after_run.newlines {
        println!();
    }
    match terminal_size() {
        None => println!("Warning: cannot determine terminal size"),
        Some((width, _)) => {
            for _ in 0..config.options.after_run.indicator_lines {
                let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
                let color = if result {
                    termcolor::Color::Green
                } else {
                    termcolor::Color::Red
                };
                stdout
                    .set_color(termcolor::ColorSpec::new().set_fg(Some(color)))
                    .unwrap();
                let text: String = "█".repeat(width.0 as usize);
                writeln!(&mut stdout, "{}", text).unwrap();
                let _ = stdout.reset(); // we really don't care about being unable to reset colors here
            }
        }
    }
    Ok(())
}

fn run_command(text: String, configuration: &config::Configuration) -> Result<bool> {
    let trigger = trigger::from_string(&text)?;
    match configuration.get_command(trigger) {
        Err(err) => match err {
            UserError::NoCommandToRepeat {} => {
                // repeat non-existing command --> don't stop, just print an error message and keep going
                let (msg, desc) = err.messages();
                println!("{}", msg);
                println!("{}", desc);
                Ok(false)
            }
            _ => Err(err),
        },
        Ok(command) => match run::run(&command) {
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
