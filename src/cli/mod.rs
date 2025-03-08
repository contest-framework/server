//! everything around CLI input

mod command;
pub mod ctrl_c;
mod exit;
mod print_error;

pub use command::Command;
pub use exit::exit;
pub use print_error::{error_color, print_error};
