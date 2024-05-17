//! manages and reads the FIFO pipe

use crate::channel::Signal;
use crate::{channel, Result, UserError};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::thread;

pub const FILE_NAME: &str = ".tertestrial.tmp";

/// A FIFO pipe
#[derive(Debug)]
pub struct Pipe {
  pub filepath: PathBuf,
}

impl Pipe {
  // creates the FIFO on the filesystem
  pub fn create(&self) -> Result<()> {
    match nix::unistd::mkfifo(&self.filepath, nix::sys::stat::Mode::S_IRWXU) {
      Ok(_) => Ok(()),
      Err(err) => match err.as_errno() {
        None => panic!("cannot determine error code"),
        Some(err_code) => match err_code {
          nix::errno::Errno::EEXIST => Err(UserError::FifoAlreadyExists {
            path: self.path_str(),
          }),
          _ => Err(UserError::FifoCannotCreate {
            path: self.filepath.to_string_lossy().to_string(),
            err: err.to_string(),
          }),
        },
      },
    }
  }

  pub fn delete(&self) -> Result<()> {
    fs::remove_file(&self.filepath).map_err(|e| UserError::FifoCannotDelete {
      err: e.to_string(),
      path: self.filepath.to_string_lossy().to_string(),
    })
  }

  pub fn open(&self) -> BufReader<File> {
    let file = File::open(&self.filepath).unwrap();
    BufReader::new(file)
  }

  /// provides the path of this pipe as a string
  pub fn path_str(&self) -> String {
    self.filepath.display().to_string()
  }
}

/// constructs a fifo pipe in the current directory
pub fn in_dir(dirpath: &Path) -> Pipe {
  Pipe {
    filepath: dirpath.join(FILE_NAME),
  }
}

pub fn listen(pipe: Pipe, sender: channel::Sender) {
  thread::spawn(move || loop {
    for line in pipe.open().lines() {
      match line {
        Ok(text) => sender
          .send(Signal::ReceivedLine(text))
          .unwrap_or_else(|err| println!("communication channel failure: {err}")),
        Err(err) => {
          sender
            .send(Signal::CannotReadPipe(err))
            .unwrap_or_else(|err| println!("communication channel failure: {err} "));
          break;
        }
      };
    }
  });
}

//
// ----------------------------------------------------------------------------
//

#[cfg(test)]
mod tests {
  use crate::client::fifo::in_dir;
  use crate::UserError;
  use std::{fs, io};

  #[test]
  fn pipe_create_does_not_exist() -> Result<(), io::Error> {
    let temp_path = tempfile::tempdir().unwrap().into_path();
    let pipe = in_dir(&temp_path);
    match pipe.create() {
      Ok(_) => {}
      _ => panic!("cannot create pipe"),
    }
    let mut files = vec![];
    for file in fs::read_dir(&temp_path)? {
      files.push(file?.path());
    }
    let want = vec![pipe.filepath];
    assert_eq!(want, files);
    fs::remove_dir_all(&temp_path)?;
    Ok(())
  }

  #[test]
  fn pipe_create_exists() -> Result<(), io::Error> {
    let temp_path = tempfile::tempdir().unwrap().into_path();
    let pipe = in_dir(&temp_path);
    match pipe.create() {
      Ok(_) => {}
      _ => panic!("cannot create first pipe"),
    }
    match pipe.create() {
      Err(UserError::FifoAlreadyExists { path: _ }) => {}
      Ok(_) => panic!("should not create second pipe"),
      Err(err) => panic!("{}", err.messages().0),
    }
    fs::remove_dir_all(&temp_path)?;
    Ok(())
  }

  #[test]
  fn pipe_delete() -> Result<(), io::Error> {
    let temp_path = tempfile::tempdir().unwrap().into_path();
    let pipe = in_dir(&temp_path);
    match pipe.create() {
      Ok(_) => {}
      _ => panic!(),
    }
    pipe.delete().unwrap();
    let mut files = vec![];
    for file in fs::read_dir(&temp_path)? {
      files.push(file?.path());
    }
    assert_eq!(0, files.len());
    fs::remove_dir_all(&temp_path).unwrap();
    Ok(())
  }
}
