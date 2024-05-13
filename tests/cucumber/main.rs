mod logic;
mod world;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use world::TertestrialWorld;

#[given(expr = "file {string} with content")]
async fn file_with_content(world: &mut TertestrialWorld, step: &Step, filename: String) {
  logic::create_file(
    &world.dir.as_ref().join(filename),
    step.docstring.as_ref().expect("no docstring"),
  )
  .await;
}

#[when(expr = "I run {string}")]
async fn start_tertestrial(world: &mut TertestrialWorld, command: String) {
  let words = shellwords::split(&command).unwrap();
  let (cmd, args) = words.split_at(1);
  if cmd != &["tertestrial"] {
    panic!("can only execute tertestrial");
  }
  logic::start_tertestrial(world, args).await;
}

#[then("it prints")]
async fn it_prints(world: &mut TertestrialWorld, step: &Step) {
  logic::verify_prints_lines(world, step.docstring.as_ref().unwrap().trim()).await;
}

#[then(expr = "it creates file {string} with content")]
async fn it_creates_file_with_content(world: &mut TertestrialWorld, step: &Step, filename: String) {
  let filepath = world.dir.as_ref().join(filename);
  let content = step.docstring.as_ref().unwrap();
  logic::verify_created_file(&filepath, content).await;
}

#[then("it exits with no output")]
async fn it_exits_with_no_output(world: &mut TertestrialWorld) {
  logic::verify_prints_text(world, "").await;
  logic::wait_for_exit(world).await;
}

#[then("it exits with this output")]
async fn it_exits_with_output(world: &mut TertestrialWorld, step: &Step) {
  logic::verify_prints_text(world, step.docstring.as_ref().unwrap().trim()).await;
  logic::wait_for_exit(world).await;
}

#[when(expr = "receiving the command {string}")]
async fn client_sends_command(world: &mut TertestrialWorld, command: String) {
  logic::send_command(command, world.dir.as_ref()).await;
}

#[given(expr = "Tertestrial is running")]
async fn tertestrial_is_running(world: &mut TertestrialWorld) {
  logic::start_tertestrial(world, &vec![]).await;
  logic::verify_prints_lines(world, "Tertestrial is online, Ctrl-C to exit").await;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  TertestrialWorld::cucumber()
    .fail_fast()
    .fail_on_skipped()
    // .max_concurrent_scenarios(Some(2))
    .run_and_exit("features/")
    .await;
}
