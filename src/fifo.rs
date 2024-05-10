//! manages and reads the FIFO pipe

use super::channel;
use super::errors::UserError;
use crate::Result;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::thread;

/// A FIFO pipe
#[derive(Debug)]
pub struct Pipe {
    pub filepath: PathBuf,
}

pub enum CreateOutcome {
    Ok(),
    AlreadyExists(String), // a pipe already exists at the given path
    OtherError(String),    // other error creating the pipe
}

impl Pipe {
    // creates the FIFO on the filesystem
    pub fn create(&self) -> CreateOutcome {
        match nix::unistd::mkfifo(&self.filepath, nix::sys::stat::Mode::S_IRWXU) {
            Ok(_) => CreateOutcome::Ok(),
            Err(err) => match err.as_errno() {
                None => panic!("cannot determine error code"),
                Some(err_code) => match err_code {
                    nix::errno::Errno::EEXIST => CreateOutcome::AlreadyExists(self.path_str()),
                    _ => CreateOutcome::OtherError(err.to_string()),
                },
            },
        }
    }

    pub fn delete(&self) -> Result<()> {
        fs::remove_file(&self.filepath)
            .map_err(|e| UserError::FifoCannotDelete { err: e.to_string() })
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
        filepath: dirpath.join(".tertestrial.tmp"),
    }
}

pub fn listen(pipe: Pipe, sender: channel::Sender) {
    thread::spawn(move || loop {
        for line in pipe.open().lines() {
            match line {
                Ok(text) => sender.send(channel::Signal::ReceivedLine(text)).unwrap(),
                Err(err) => {
                    sender.send(channel::Signal::CannotReadPipe(err)).unwrap();
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
    use crate::fifo::{in_dir, CreateOutcome};
    use std::{fs, io};

    #[test]
    fn pipe_create_does_not_exist() -> Result<(), io::Error> {
        let temp_path = tempfile::tempdir().unwrap().into_path();
        let pipe = in_dir(&temp_path);
        match pipe.create() {
            CreateOutcome::Ok() => {}
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
            CreateOutcome::Ok() => {}
            _ => panic!("cannot create first pipe"),
        }
        match pipe.create() {
            CreateOutcome::AlreadyExists(_) => {}
            CreateOutcome::Ok() => panic!("should not create second pipe"),
            CreateOutcome::OtherError(err) => panic!("{}", err),
        }
        fs::remove_dir_all(&temp_path)?;
        Ok(())
    }

    #[test]
    fn pipe_delete() -> Result<(), io::Error> {
        let temp_path = tempfile::tempdir().unwrap().into_path();
        let pipe = in_dir(&temp_path);
        match pipe.create() {
            CreateOutcome::Ok() => {}
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
