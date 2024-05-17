use crate::fifo;
use std::fs;

pub fn exit(err: String) {
  println!("ERROR: {}", err);
  let _ = fs::remove_file(fifo::FILE_NAME);
  std::process::exit(1);
}
