use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::process::Stdio;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

#[derive(Debug, World)]
// Accepts both sync/async and fallible/infallible functions.
#[world(init = Self::new)]
pub struct CukeWorld {
  cmd: Option<Child>,
  stdin: Option<BufWriter<ChildStdin>>,
  stdout: Option<BufReader<ChildStdout>>,
  workspace: TempDir,
}

impl CukeWorld {
  fn new() -> Self {
    let root = tempfile::tempdir().unwrap();
    Self {
      workspace: root,
      cmd: None,
      stdin: None,
      stdout: None,
    }
  }
}

#[given(expr = "file {string} with content")]
async fn file_with_content(world: &mut CukeWorld, step: &Step, filename: String) {
  let file_path = world.workspace.as_ref().join(filename);
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
    .current_dir(world.workspace.as_ref())
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

#[when("a client sends the command")]
async fn client_sends_command(world: &mut CukeWorld, step: &Step) {
  let command = step.docstring.as_ref().unwrap().trim();
  let fifo = world.fifo.write();
}

#[then("it exits")]
async fn it_exits(world: &mut CukeWorld) {
  let cmd = world.cmd.as_mut().unwrap();
  let o = cmd.wait().await.unwrap();
  assert_eq!(o.code().unwrap(), 0);
}

#[then("it prints")]
async fn it_prints(world: &mut CukeWorld, step: &Step) {
  let want = step.docstring.as_ref().unwrap().trim();
  let reader = world.stdout.as_mut().unwrap();
  let mut output = String::new();
  let mut have = String::new();
  while have.is_empty() {
    reader.read_line(&mut output).await.unwrap();
    output.trim().clone_into(&mut have);
  }
  assert_eq!(&have, want);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  CukeWorld::cucumber()
    .fail_fast()
    .max_concurrent_scenarios(Some(1))
    .run_and_exit("features/")
    .await;
}
