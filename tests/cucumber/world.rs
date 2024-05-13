use cucumber::World;
use tempfile::TempDir;
use tokio::process::ChildStdout;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TertestrialWorld {
  /// the subprocess executing Tertestrial
  pub subprocess: Option<RunningProcess>,
  /// the directory containing the source code that Tertestrial should check
  pub dir: TempDir,
}

#[derive(Debug)]
pub struct RunningProcess {
  pub cmd: tokio::process::Child,
  pub stdout: tokio::io::BufReader<ChildStdout>,
}

impl TertestrialWorld {
  fn new() -> Self {
    Self {
      dir: tempfile::tempdir().unwrap(),
      subprocess: None,
    }
  }
}
