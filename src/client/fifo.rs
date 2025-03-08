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
  pub fn delete(&self) -> Result<()> {
    fs::remove_file(&self.filepath).map_err(|e| UserError::FifoCannotDelete {
      err: e.to_string(),
      path: self.filepath.to_string_lossy().to_string(),
    })
  }

  /// constructs a fifo pipe in the current directory
  #[must_use]
  pub fn in_dir(dirpath: &Path) -> Result<Fifo> {
    let full_path = dirpath.join(FILE_NAME);
    if let Err(err) = nix::unistd::mkfifo(&full_path, nix::sys::stat::Mode::S_IRWXU) {
      return match err.as_errno() {
        Some(nix::errno::Errno::EEXIST) => Err(UserError::FifoAlreadyExists {
          path: full_path.to_string_lossy().to_string(),
        }),
        _ => Err(UserError::FifoCannotCreate {
          path: full_path.to_string_lossy().to_string(),
          err: err.to_string(),
        }),
      };
    }
    Ok(Fifo { filepath: full_path })
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

impl Drop for Fifo {
  fn drop(&mut self) {
    println!("deleting FIFO from disk");
    self.delete().unwrap()
  }
}

pub fn listen(pipe: Fifo, sender: channel::Sender) {
  thread::spawn(move || {
    loop {
      let reader = pipe.open().unwrap_or_else(|err| cli::exit(&err.messages().0));
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
    let fifo = Fifo::in_dir(&temp_path).unwrap();
    let mut files = vec![];
    for file in fs::read_dir(&temp_path)? {
      files.push(file?.path());
    }
    let want = vec![fifo.filepath.clone()];
    assert_eq!(want, files);
    Ok(())
  }

  #[test]
  fn pipe_create_exists() -> Result<(), String> {
    let temp_dir = tempfile::tempdir().unwrap();
    let _fifo1 = Fifo::in_dir(temp_dir.path()).unwrap();
    let fifo2 = Fifo::in_dir(temp_dir.path());
    match fifo2 {
      Err(UserError::FifoAlreadyExists { path: _ }) => Ok(()),
      Err(err) => Err(err.messages().0),
      Ok(_) => Err(S("should not create second pipe")),
    }
  }

  #[test]
  fn pipe_delete() -> Result<(), UserError> {
    let temp_dir = tempfile::tempdir().unwrap();
    let fifo = Fifo::in_dir(temp_dir.path())?;
    fifo.delete().unwrap();
    let file_count = fs::read_dir(temp_dir.path()).unwrap().count();
    assert_eq!(0, file_count);
    Ok(())
  }
}
