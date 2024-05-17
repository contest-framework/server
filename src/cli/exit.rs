use crate::fifo;
use std::fs;

pub fn exit(err: &str) -> ! {
  println!("ERROR: {err}");
  let _ = fs::remove_file(fifo::FILE_NAME);
  std::process::exit(1);
}
