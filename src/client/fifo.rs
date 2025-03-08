//! manages and reads the FIFO pipe

use crate::channel::Signal;
use crate::{Result, UserError, channel, cli};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread;

pub const FILE_NAME: &str = ".contest.tmp";

/// A FIFO pipe
#[derive(Debug)]
pub struct Fifo {
  pub filepath: PathBuf,
}

impl Fifo {
  // creates the FIFO on the filesystem
  fn create(&self) -> Result<()> {
    match nix::unistd::mkfifo(&self.filepath, nix::sys::stat::Mode::S_IRWXU) {
      Ok(()) => Ok(()),
      Err(err) => match err.as_errno() {
        Some(nix::errno::Errno::EEXIST) => Err(UserError::FifoAlreadyExists { path: self.path_str() }),
        _ => Err(UserError::FifoCannotCreate {
          path: self.filepath.to_string_lossy().to_string(),
          err: err.to_string(),
        }),
      },
    }
  }

  /// constructs a fifo pipe in the current directory
  #[must_use]
  pub fn in_dir(dirpath: &Path) -> Self {
    Fifo {
      filepath: dirpath.join(FILE_NAME),
    }
  }

  pub fn listen(self, sender: channel::Sender) -> Result<()> {
    self.create()?;
    thread::spawn(move || {
      loop {
        let reader = self.open().unwrap_or_else(|err| cli::exit(&err.messages().0));
        for line in reader.lines() {
          match line {
            Ok(text) => sender
              .send(Signal::ReceivedLine(text))
              .unwrap_or_else(|err| println!("communication channel failure: {err}")),
            Err(err) => cli::exit(&err.to_string()),
          };
        }
      }
    });
    Ok(())
  }

  pub fn delete(&self) -> Result<()> {
    fs::remove_file(&self.filepath).map_err(|e| UserError::FifoCannotDelete {
      err: e.to_string(),
      path: self.filepath.to_string_lossy().to_string(),
    })
  }

  pub fn open(&self) -> Result<BufReader<File>> {
    let file = File::open(&self.filepath).map_err(|err| UserError::FifoCannotOpen { err: err.to_string() })?;
    Ok(BufReader::new(file))
  }

  /// provides the path of this pipe as a string
  #[must_use]
  pub fn path_str(&self) -> String {
    self.filepath.display().to_string()
  }
}

#[cfg(test)]
mod tests {
  use crate::UserError;
  use crate::client::Fifo;
  use big_s::S;
  use std::{fs, io};

  #[test]
  fn pipe_create_does_not_exist() -> Result<(), io::Error> {
    let temp_path = tempfile::tempdir().unwrap().into_path();
    let pipe = Fifo::in_dir(&temp_path);
    pipe.create().unwrap();
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
  fn pipe_create_exists() -> Result<(), String> {
    let temp_dir = tempfile::tempdir().unwrap();
    let pipe = Fifo::in_dir(temp_dir.path());
    pipe.create().unwrap();
    match pipe.create() {
      Err(UserError::FifoAlreadyExists { path: _ }) => Ok(()),
      Err(err) => Err(err.messages().0),
      Ok(()) => Err(S("should not create second pipe")),
    }
  }

  #[test]
  fn pipe_delete() -> Result<(), UserError> {
    let temp_dir = tempfile::tempdir().unwrap();
    let pipe = Fifo::in_dir(temp_dir.path());
    pipe.create()?;
    pipe.delete().unwrap();
    let file_count = fs::read_dir(temp_dir.path()).unwrap().count();
    assert_eq!(0, file_count);
    Ok(())
  }
}
