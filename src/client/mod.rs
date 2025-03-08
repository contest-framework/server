//! everything around communicating with the client

pub mod fifo;
mod fifo_data;
pub mod trigger;

pub use fifo::Fifo;
pub use fifo_data::FifoTrigger;
pub use trigger::Trigger;
