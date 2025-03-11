//! capture and handle Ctrl-C

use crate::{channel, cli};

/// captures Ctrl-C and messages it as a `Signal::Exit` message via the given sender
pub(crate) fn handle(sender: channel::Sender) {
  ctrlc::set_handler(move || {
    sender.send(channel::Signal::Exit).unwrap_or_else(|err| cli::exit(&err.to_string()));
  })
  .unwrap_or_else(|err| cli::exit(&err.to_string()));
}
