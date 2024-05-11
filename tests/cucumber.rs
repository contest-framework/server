use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::process::Stdio;
use tempfile::TempDir;
use tertestrial::client::fifo;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct CukeWorld {
  cmd: Option<Child>,
  stdin: Option<BufWriter<ChildStdin>>,
  stdout: Option<BufReader<ChildStdout>>,
  dir: TempDir,
}

impl CukeWorld {
  fn new() -> Self {
    Self {
      dir: tempfile::tempdir().unwrap(),
      cmd: None,
      stdin: None,
      stdout: None,
    }
  }
}

#[given(expr = "file {string} with content")]
async fn file_with_content(world: &mut CukeWorld, step: &Step, filename: String) {
  let file_path = world.dir.as_ref().join(filename);
  let mut file = File::create(file_path).await.unwrap();
  let Some(content) = step.docstring.as_ref() else {
    panic!("no docstring");
  };
  file.write_all(content.as_bytes()).await.unwrap();
}

#[given("I start Tertestrial")]
async fn tertestrial_running(world: &mut CukeWorld) {
  let cwd = std::env::current_dir().unwrap();
  let tertestrial_path = cwd.join("target").join("debug").join("tertestrial");
  let mut cmd = Command::new(tertestrial_path)
    .current_dir(world.dir.as_ref())
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .kill_on_drop(true)
    .spawn()
    .unwrap();
  let stdin = cmd.stdin.take().expect("Failed to open subshell stdin");
  let stdout = cmd.stdout.take().expect("Failed to open subshell stdout");
  let stdin_writer = BufWriter::new(stdin);
  let stdout_writer = BufReader::new(stdout);
  world.cmd = Some(cmd);
  world.stdin = Some(stdin_writer);
  world.stdout = Some(stdout_writer);
}

#[when(expr = "a client sends the command {string}")]
async fn client_sends_command(world: &mut CukeWorld, command: String) {
  let fifo_path = world.dir.as_ref().join(fifo::FILE_NAME);
  if fs::metadata(&fifo_path).await.is_err() {
    panic!("FIFO not found")
  }
  let mut fifo = OpenOptions::new()
    .write(true)
    .open(&fifo_path)
    .await
    .unwrap();
  fifo.write_all(command.as_bytes()).await.unwrap();
}

#[then("it exits")]
async fn it_exits(world: &mut CukeWorld) {
  let cmd = world.cmd.as_mut().unwrap();
  let o = cmd.wait().await.unwrap();
  assert_eq!(o.code().unwrap(), 0);
}

#[then("it prints")]
async fn it_prints(world: &mut CukeWorld, step: &Step) {
  let want_text = step.docstring.as_ref().unwrap().trim();
  let reader = world.stdout.as_mut().unwrap();
  for want_line in want_text.lines() {
    let mut output = String::new();
    let mut have = String::with_capacity(want_line.len());
    while have.is_empty() {
      reader.read_line(&mut output).await.unwrap();
      output.trim().clone_into(&mut have);
    }
    assert_eq!(&have, want_line);
  }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  CukeWorld::cucumber()
    .fail_fast()
    // .max_concurrent_scenarios(Some(2))
    .run_and_exit("features/")
    .await;
}
