//! capture and handle Ctrl-C

use crate::channel;

/// captures Ctrl-C and messages it as a Signal::Exit message via the given sender
pub fn handle(sender: channel::Sender) {
  ctrlc::set_handler(move || {
    sender.send(channel::Signal::Exit).unwrap();
  })
  .unwrap();
}
