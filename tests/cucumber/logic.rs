use crate::world::{RunningProcess, TertestrialWorld};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tertestrial::client::fifo;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

pub async fn create_file(path: &Path, content: impl AsRef<str>) {
  let mut file = File::create(path).await.unwrap();
  file.write_all(content.as_ref().as_bytes()).await.unwrap();
}

pub fn fifo_path(workspace: &Path) -> PathBuf {
  workspace.join(fifo::FILE_NAME)
}

async fn ensure_fifo_exists(fifo_path: &Path) {
  let metadata = fs::metadata(&fifo_path).await.expect("FIFO not found");
  assert!(metadata.file_type().is_fifo());
}

pub async fn send_command(command: String, workspace: &Path) {
  let fifo_path = fifo_path(workspace);
  ensure_fifo_exists(&fifo_path).await;
  let mut fifo = OpenOptions::new()
    .write(true)
    .open(&fifo_path)
    .await
    .unwrap();
  fifo.write_all(command.as_bytes()).await.unwrap();
}

pub async fn start_tertestrial(world: &mut TertestrialWorld, args: &[String]) {
  let cwd = std::env::current_dir().unwrap();
  let tertestrial_path = cwd.join("target").join("debug").join("tertestrial");
  let mut cmd = Command::new(tertestrial_path)
    .args(args)
    .current_dir(world.dir.as_ref())
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .kill_on_drop(true)
    .spawn()
    .unwrap();
  let stdout = cmd.stdout.take().expect("Failed to open subshell stdout");
  let stdout_writer = BufReader::new(stdout);
  world.subprocess = Some(RunningProcess {
    cmd,
    stdout: stdout_writer,
  });
}

pub async fn verify_created_file(file_path: &Path, want: &str) {
  let have = fs::read_to_string(file_path).await.unwrap();
  pretty::assert_eq!(have.trim(), want.trim());
}

pub async fn verify_prints_lines(world: &mut TertestrialWorld, output: &str) {
  let subprocess = world.subprocess.as_mut().unwrap();
  for want in output.lines() {
    let mut output = String::new();
    let mut have = String::with_capacity(want.len());
    while have.is_empty() {
      subprocess.stdout.read_line(&mut output).await.unwrap();
      output.trim().clone_into(&mut have);
    }
    assert_eq!(&have, want);
  }
}

pub async fn verify_prints_text(world: &mut TertestrialWorld, want: &str) {
  let subprocess = world.subprocess.as_mut().unwrap();
  let mut have = Vec::<u8>::with_capacity(want.len());
  subprocess.stdout.read_to_end(&mut have).await.unwrap();
  let have = String::from_utf8(have).unwrap();
  pretty::assert_eq!(have.trim(), want.trim());
}

pub async fn wait_for_exit(world: &mut TertestrialWorld) {
  let subprocess = world.subprocess.as_mut().unwrap();
  let exit_status = subprocess.cmd.wait().await.unwrap();
  assert_eq!(exit_status.code().unwrap(), 0);
}
