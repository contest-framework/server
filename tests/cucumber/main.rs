mod logic;
mod world;

use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use world::ContestWorld;

#[given(expr = "file {string} with content")]
async fn file_with_content(world: &mut ContestWorld, step: &Step, filename: String) {
  logic::create_file(&world.dir.as_ref().join(filename), step.docstring.as_ref().expect("no docstring")).await;
}

#[when(expr = "I run {string}")]
async fn start_contest(world: &mut ContestWorld, command: String) {
  let words = shellwords::split(&command).unwrap();
  let (cmd, args) = words.split_at(1);
  if cmd != &["contest"] {
    panic!("can only execute contest");
  }
  logic::start_contest(world, args).await;
}

#[then("it prints")]
async fn it_prints(world: &mut ContestWorld, step: &Step) {
  logic::verify_prints_lines(world, step.docstring.as_ref().unwrap().trim()).await;
}

#[then(expr = "it creates file {string} with content")]
async fn it_creates_file_with_content(world: &mut ContestWorld, step: &Step, filename: String) {
  let filepath = world.dir.as_ref().join(filename);
  let content = step.docstring.as_ref().unwrap();
  logic::verify_created_file(&filepath, content).await;
}

#[then("it exits with no output")]
async fn it_exits_with_no_output(world: &mut ContestWorld) {
  logic::verify_prints_text(world, "").await;
  logic::wait_for_exit(world, 0).await;
}

#[then("it exits with this output")]
async fn it_exits_with_output(world: &mut ContestWorld, step: &Step) {
  logic::verify_prints_text(world, step.docstring.as_ref().unwrap().trim()).await;
  logic::wait_for_exit(world, 0).await;
}

#[then("it fails with this output")]
async fn it_fails_with_output(world: &mut ContestWorld, step: &Step) {
  logic::verify_prints_text(world, step.docstring.as_ref().unwrap().trim()).await;
  logic::wait_for_exit(world, 1).await;
}

#[when(expr = "receiving the command {string}")]
async fn client_sends_command(world: &mut ContestWorld, command: String) {
  logic::send_command(command, world.dir.as_ref()).await;
}

#[then(expr = "the server stops running")]
async fn server_no_longer_running(world: &mut ContestWorld) {
  let subprocess = world.subprocess.as_mut().unwrap();
  let exit_status = subprocess.cmd.wait().await.unwrap();
  assert!(exit_status.success());
}

#[given(expr = "Contest is running")]
async fn contest_is_running(world: &mut ContestWorld) {
  logic::start_contest(world, &vec![]).await;
  logic::verify_prints_lines(world, "Contest is online, Ctrl-C to exit").await;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  ContestWorld::cucumber().fail_fast().fail_on_skipped().run_and_exit("features/").await;
}
