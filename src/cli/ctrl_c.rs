//! capture and handle Ctrl-C

use crate::channel;
use crate::cli;

/// captures Ctrl-C and messages it as a Signal::Exit message via the given sender
pub fn handle(sender: channel::Sender) {
  if let Err(err) = ctrlc::set_handler(move || {
    if let Err(err) = sender.send(channel::Signal::Exit) {
      cli::exit(&err.to_string());
    }
  }) {
    cli::exit(&err.to_string());
  }
}
