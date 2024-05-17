//! everything around CLI input

pub mod ctrl_c;
mod exit;
mod print_error;

pub use exit::exit;
pub use print_error::{error_color, print_error};
