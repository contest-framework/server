use std::process::Stdio;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, BufReader, BufWriter};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

#[derive(Debug, World)]
// Accepts both sync/async and fallible/infallible functions.
#[world(init = Self::new)]
pub struct AnimalWorld {
  cat: Cat,
  workspace: TempDir,
  cmd: Option<Child>,
  stdin: Option<BufWriter<ChildStdin>>,
  stdout: Option<BufReader<ChildStdout>>,
}

impl AnimalWorld {
  fn new() -> Self {
    let root = tempfile::tempdir().unwrap();
    Self {
      cat: Cat { hungry: true },
      workspace: root,
      cmd: None,
      stdin: None,
      stdout: None,
    }
  }
}

#[given("Tertestrial is running")]
async fn tertestrial_running(world: &mut AnimalWorld) {
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

#[then("it prints:")]
async fn it_prints(world: &mut AnimalWorld, step: &Step) {
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

// -------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Cat {
  pub hungry: bool,
}

impl Cat {
  fn feed(&mut self) {
    self.hungry = false;
  }
}

#[given("a hungry cat")]
fn hungry_cat(world: &mut AnimalWorld) {
  world.cat.hungry = true;
}

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
  world.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
  assert!(!world.cat.hungry);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  AnimalWorld::cucumber()
    .fail_fast()
    .max_concurrent_scenarios(Some(1))
    .run_and_exit("features/")
    .await;
}
